macro_rules! load_metadata_with {
    () => {
        fn load_metadata_with (path : & Path , f : impl for < 'a > FnOnce (& 'a [u8]) -> Result < & 'a [u8] , String > ,) -> Result < OwnedSlice , String > { let file = File :: open (path) . map_err (| e | format ! ("failed to open file '{}': {}" , path . display () , e)) ? ; unsafe { Mmap :: map (file) } . map_err (| e | format ! ("failed to mmap file '{}': {}" , path . display () , e)) . and_then (| mmap | try_slice_owned (mmap , | mmap | f (mmap))) }
    };
}

load_metadata_with!();