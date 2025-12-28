macro_rules! deps {
    () => {
        Error!();
        Find!();
        Change!();
        Header!();
        Entry!();
        Kind!();
        LookupRefDeltaObjectsIter!();
        Item!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < I , Find > LookupRefDeltaObjectsIter < I , Find > where I : Iterator < Item = Result < input :: Entry , input :: Error > > , Find : gix_object :: Find , { # [doc = " Create a new instance wrapping `iter` and using `lookup` as function to retrieve objects that will serve as bases"] # [doc = " for ref deltas seen while traversing `iter`."] pub fn new (iter : I , lookup : Find) -> Self { LookupRefDeltaObjectsIter { inner : iter , lookup , error : false , inserted_entry_length_at_offset : Vec :: new () , inserted_entries_length_in_bytes : 0 , next_delta : None , buf : Vec :: new () , } } fn shifted_pack_offset (& self , pack_offset : u64) -> u64 { let new_ofs = pack_offset as i64 + self . inserted_entries_length_in_bytes ; new_ofs . try_into () . expect ("offset value is never becomes negative") } # [doc = " positive `size_change` values mean an object grew or was more commonly, was inserted. Negative values"] # [doc = " mean the object shrunk, usually because there header changed from ref-deltas to ofs deltas."] fn track_change (& mut self , shifted_pack_offset : u64 , pack_offset : u64 , size_change : i64 , oid : Option < ObjectId >) { if size_change == 0 { return ; } self . inserted_entry_length_at_offset . push (Change { shifted_pack_offset , pack_offset , size_change_in_bytes : size_change , oid : oid . unwrap_or_else (| | gix_hash :: Kind :: Sha1 . null ()) , }) ; self . inserted_entries_length_in_bytes += size_change ; } fn shift_entry_and_point_to_base_by_offset (& mut self , entry : & mut input :: Entry , base_distance : u64) { let pack_offset = entry . pack_offset ; entry . pack_offset = self . shifted_pack_offset (pack_offset) ; entry . header = Header :: OfsDelta { base_distance } ; let previous_header_size = entry . header_size ; entry . header_size = entry . header . size (entry . decompressed_size) as u16 ; let change = i64 :: from (entry . header_size) - i64 :: from (previous_header_size) ; entry . crc32 = Some (entry . compute_crc32 ()) ; self . track_change (entry . pack_offset , pack_offset , change , None) ; } }
    };
}

impl_157!()