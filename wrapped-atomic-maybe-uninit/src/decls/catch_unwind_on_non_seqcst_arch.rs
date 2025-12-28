macro_rules! catch_unwind_on_non_seqcst_arch {
    () => {
        pub (crate) fn catch_unwind_on_non_seqcst_arch (pat : & str , f : impl Fn ()) { if ! is_panic_abort () { match std :: panic :: catch_unwind (std :: panic :: AssertUnwindSafe (f)) { Ok (()) => { } Err (msg) => { let msg = msg . downcast_ref :: < std :: string :: String > () . cloned () . unwrap_or_else (| | msg . downcast_ref :: < & 'static str > () . copied () . unwrap () . into ()) ; assert ! (msg . contains (pat) , "{}" , msg) ; } } } }
    };
}

catch_unwind_on_non_seqcst_arch!()