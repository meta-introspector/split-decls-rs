macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! acquire_shallow_lock {
    () => {
        deps!();
        fn acquire_shallow_lock (shallow_file : & Path) -> Result < gix_lock :: File , Error > { gix_lock :: File :: acquire_to_update_resource (shallow_file , gix_lock :: acquire :: Fail :: Immediately , None) . map_err (Into :: into) }
    };
}

acquire_shallow_lock!();