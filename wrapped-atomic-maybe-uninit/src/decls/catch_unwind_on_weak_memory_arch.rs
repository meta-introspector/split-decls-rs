macro_rules! catch_unwind_on_weak_memory_arch {
    () => {
        pub (crate) fn catch_unwind_on_weak_memory_arch (pat : & str , f : impl Fn ()) { if cfg ! (any (target_arch = "x86" , target_arch = "x86_64" , target_arch = "s390x" , target_arch = "sparc" , target_arch = "sparc64" ,)) { f () ; } else if ! is_panic_abort () { match std :: panic :: catch_unwind (std :: panic :: AssertUnwindSafe (f)) { Ok (()) => { } Err (msg) => { let msg = msg . downcast_ref :: < std :: string :: String > () . cloned () . unwrap_or_else (| | msg . downcast_ref :: < & 'static str > () . copied () . unwrap () . into ()) ; assert ! (msg . contains (pat) , "{}" , msg) ; } } } }
    };
}

catch_unwind_on_weak_memory_arch!()