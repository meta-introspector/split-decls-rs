macro_rules! deps {
    () => {
        Entry!();
        Error!();
        SafetyCheck!();
        Kind!();
    };
}

macro_rules! process_entry {
    () => {
        deps!();
        # [allow (clippy :: too_many_arguments)] fn process_entry < E > (check : SafetyCheck , object_kind : gix_object :: Kind , decompressed : & [u8] , index_entry : & index :: Entry , pack_entry_crc32 : impl FnOnce () -> u32 , progress : & dyn Progress , processor : & mut impl FnMut (gix_object :: Kind , & [u8] , & index :: Entry , & dyn Progress) -> Result < () , E > ,) -> Result < () , Error < E > > where E : std :: error :: Error + Send + Sync + 'static , { if check . object_checksum () { gix_object :: Data :: new (object_kind , decompressed) . verify_checksum (& index_entry . oid) . map_err (| source | Error :: PackObjectVerify { offset : index_entry . pack_offset , source , }) ? ; if let Some (desired_crc32) = index_entry . crc32 { let actual_crc32 = pack_entry_crc32 () ; if actual_crc32 != desired_crc32 { return Err (Error :: Crc32Mismatch { actual : actual_crc32 , expected : desired_crc32 , offset : index_entry . pack_offset , kind : object_kind , }) ; } } } processor (object_kind , decompressed , index_entry , progress) . map_err (Error :: Processor) }
    };
}

process_entry!();