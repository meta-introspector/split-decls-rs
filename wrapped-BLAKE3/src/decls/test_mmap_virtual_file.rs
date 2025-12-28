macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! test_mmap_virtual_file {
    () => {
        deps!();
        # [test] # [cfg (feature = "mmap")] # [cfg (target_os = "linux")] fn test_mmap_virtual_file () -> Result < () , std :: io :: Error > { let virtual_filepath = "/proc/version" ; let mut mmap_hasher = crate :: Hasher :: new () ; mmap_hasher . update_mmap (virtual_filepath) ? ; let mut read_hasher = crate :: Hasher :: new () ; read_hasher . update_reader (std :: fs :: File :: open (virtual_filepath) ?) ? ; assert_eq ! (mmap_hasher . finalize () , read_hasher . finalize ()) ; Ok (()) }
    };
}

test_mmap_virtual_file!();