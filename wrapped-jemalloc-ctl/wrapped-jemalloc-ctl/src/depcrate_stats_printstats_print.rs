// Generated macro for stats_print (function)
macro_rules! Depcrate_stats_printstats_print {
() => {
// Module: crate::stats_print
// Provides: {"stats_print"}
// Dependencies: {}
# [doc = " Writes allocator statistics."] # [doc = ""] # [doc = " The information is the same that can be retrieved by the individual lookup methods in this"] # [doc = " crate, but all done at once."] # [cfg_attr (feature = "cargo-clippy" , allow (clippy :: cast_possible_wrap))] pub fn stats_print < W > (writer : W , options : Options) -> io :: Result < () > where W : Write , { unsafe { let mut state = State { writer , error : Ok (()) , panic : Ok (()) , } ; let mut opts = [0 ; 8] ; let mut i = 0 ; if options . json_format { opts [i] = b'J' as c_char ; i += 1 ; } if options . skip_constants { opts [i] = b'g' as c_char ; i += 1 ; } if options . skip_merged_arenas { opts [i] = b'm' as c_char ; i += 1 ; } if options . skip_per_arena { opts [i] = b'a' as c_char ; i += 1 ; } if options . skip_bin_size_classes { opts [i] = b'b' as c_char ; i += 1 ; } if options . skip_large_size_classes { opts [i] = b'l' as c_char ; i += 1 ; } if options . skip_mutex_statistics { opts [i] = b'x' as c_char ; i += 1 ; } opts [i] = 0 ; tikv_jemalloc_sys :: malloc_stats_print (Some (callback :: < W >) , & mut state as * mut _ as * mut c_void , opts . as_ptr () ,) ; if let Err (e) = state . panic { panic :: resume_unwind (e) ; } state . error } }
};
}
