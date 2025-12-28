macro_rules! deps {
    () => {
        StringTableBuilder!();
        StringId!();
        SerializableString!();
        SerializationSink!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl StringTableBuilder { pub fn new (data_sink : Arc < SerializationSink > , index_sink : Arc < SerializationSink > ,) -> Result < StringTableBuilder , Box < dyn Error + Send + Sync > > { write_file_header (& mut data_sink . as_std_write () , FILE_MAGIC_STRINGTABLE_DATA) ? ; write_file_header (& mut index_sink . as_std_write () , FILE_MAGIC_STRINGTABLE_INDEX) ? ; Ok (StringTableBuilder { data_sink , index_sink , }) } # [doc = " Creates a mapping so that `virtual_id` will resolve to the contents of"] # [doc = " `concrete_id` when reading the string table."] pub fn map_virtual_to_concrete_string (& self , virtual_id : StringId , concrete_id : StringId) { assert ! (virtual_id . 0 <= MAX_USER_VIRTUAL_STRING_ID) ; serialize_index_entry (& * self . index_sink , virtual_id , concrete_id . to_addr ()) ; } pub fn bulk_map_virtual_to_single_concrete_string < I > (& self , virtual_ids : I , concrete_id : StringId ,) where I : Iterator < Item = StringId > + ExactSizeIterator , { type MappingEntry = [u64 ; 2] ; assert ! (std :: mem :: size_of ::< MappingEntry > () == 16) ; let to_addr_le = concrete_id . to_addr () . 0 . to_le () ; let serialized : Vec < MappingEntry > = virtual_ids . map (| from | { let id = from . 0 ; assert ! (id <= MAX_USER_VIRTUAL_STRING_ID) ; [id . to_le () , to_addr_le] }) . collect () ; let num_bytes = serialized . len () * std :: mem :: size_of :: < MappingEntry > () ; let byte_ptr = serialized . as_ptr () as * const u8 ; let bytes = unsafe { std :: slice :: from_raw_parts (byte_ptr , num_bytes) } ; self . index_sink . write_bytes_atomic (bytes) ; } pub fn alloc_metadata < STR : SerializableString + ? Sized > (& self , s : & STR) { let concrete_id = self . alloc (s) ; let virtual_id = StringId (METADATA_STRING_ID) ; assert ! (virtual_id . is_virtual ()) ; serialize_index_entry (& * self . index_sink , virtual_id , concrete_id . to_addr ()) ; } pub fn alloc < STR : SerializableString + ? Sized > (& self , s : & STR) -> StringId { let size_in_bytes = s . serialized_size () ; let addr = self . data_sink . write_atomic (size_in_bytes , | mem | { s . serialize (mem) ; }) ; StringId :: from_addr (addr) } }
    };
}

impl_108!()