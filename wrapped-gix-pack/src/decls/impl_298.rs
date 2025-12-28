macro_rules! deps {
    () => {
        Item!();
        PackIndex!();
        PrefixLookupResult!();
        Offset!();
        Entry!();
        Find!();
        EntryIndex!();
        File!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl File { # [doc = " Return the object id at the given `index`, which ranges from 0 to [File::num_objects()]."] pub fn oid_at_index (& self , index : EntryIndex) -> & gix_hash :: oid { debug_assert ! (index < self . num_objects , "index out of bounds") ; let index : usize = index as usize ; let start = self . lookup_ofs + index * self . hash_len ; gix_hash :: oid :: from_bytes_unchecked (& self . data [start ..] [.. self . hash_len]) } # [doc = " Given a `prefix`, find an object that matches it uniquely within this index and return `Some(Ok(entry_index))`."] # [doc = " If there is more than one object matching the object `Some(Err(())` is returned."] # [doc = ""] # [doc = " Finally, if no object matches the index, the return value is `None`."] # [doc = ""] # [doc = " Pass `candidates` to obtain the set of entry-indices matching `prefix`, with the same return value as"] # [doc = " one would have received if it remained `None`. It will be empty if no object matched the `prefix`."] # [doc = ""] pub fn lookup_prefix (& self , prefix : gix_hash :: Prefix , candidates : Option < & mut Range < EntryIndex > > ,) -> Option < PrefixLookupResult > { crate :: index :: access :: lookup_prefix (prefix , candidates , & self . fan , & | idx | self . oid_at_index (idx) , self . num_objects ,) } # [doc = " Find the index ranging from 0 to [File::num_objects()] that belongs to data associated with `id`, or `None` if it wasn't found."] # [doc = ""] # [doc = " Use this index for finding additional information via [`File::pack_id_and_pack_offset_at_index()`]."] pub fn lookup (& self , id : impl AsRef < gix_hash :: oid >) -> Option < EntryIndex > { crate :: index :: access :: lookup (id . as_ref () , & self . fan , & | idx | self . oid_at_index (idx)) } # [doc = " Given the `index` ranging from 0 to [File::num_objects()], return the pack index and its absolute offset into the pack."] # [doc = ""] # [doc = " The pack-index refers to an entry in the [`index_names`][File::index_names()] list, from which the pack can be derived."] pub fn pack_id_and_pack_offset_at_index (& self , index : EntryIndex) -> (PackIndex , data :: Offset) { const OFFSET_ENTRY_SIZE : usize = 4 + 4 ; let index = index as usize ; let start = self . offsets_ofs + index * OFFSET_ENTRY_SIZE ; const HIGH_BIT : u32 = 1 << 31 ; let pack_index = crate :: read_u32 (& self . data [start ..] [.. 4]) ; let offset = & self . data [start + 4 ..] [.. 4] ; let ofs32 = crate :: read_u32 (offset) ; let pack_offset = if (ofs32 & HIGH_BIT) == HIGH_BIT { if let Some (offsets_64) = self . large_offsets_ofs { let from = offsets_64 + (ofs32 ^ HIGH_BIT) as usize * 8 ; crate :: read_u64 (& self . data [from ..] [.. 8]) } else { u64 :: from (ofs32) } } else { u64 :: from (ofs32) } ; (pack_index , pack_offset) } # [doc = " Return an iterator over all entries within this file."] pub fn iter (& self) -> impl Iterator < Item = Entry > + '_ { (0 .. self . num_objects) . map (move | idx | { let (pack_index , pack_offset) = self . pack_id_and_pack_offset_at_index (idx) ; Entry { oid : self . oid_at_index (idx) . to_owned () , pack_offset , pack_index , } }) } }
    };
}

impl_298!();