macro_rules! deps {
    () => {
        EntryRange!();
        SharedTempFile!();
    };
}

macro_rules! new_pack_file_resolver {
    () => {
        deps!();
        # [allow (clippy :: type_complexity)] fn new_pack_file_resolver (data_file : SharedTempFile ,) -> io :: Result < (impl Fn (data :: EntryRange , & memmap2 :: Mmap) -> Option < & [u8] > + Send + Clone , memmap2 :: Mmap ,) > { let mut guard = data_file . lock () ; guard . flush () ? ; let mapped_file = crate :: mmap :: read_only (& guard . get_mut () . with_mut (| f | f . path () . to_owned ()) ?) ? ; Ok ((resolve_entry , mapped_file)) }
    };
}

new_pack_file_resolver!()