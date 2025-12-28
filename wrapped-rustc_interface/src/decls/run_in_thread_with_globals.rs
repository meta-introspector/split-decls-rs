macro_rules! run_in_thread_with_globals {
    () => {
        fn run_in_thread_with_globals < F : FnOnce (CurrentGcx , Arc < Proxy >) -> R + Send , R : Send > (thread_stack_size : usize , edition : Edition , sm_inputs : SourceMapInputs , extra_symbols : & [& 'static str] , f : F ,) -> R { let builder = thread :: Builder :: new () . name ("rustc" . to_string ()) . stack_size (thread_stack_size) ; thread :: scope (| s | { let r = builder . spawn_scoped (s , move | | { rustc_span :: create_session_globals_then (edition , extra_symbols , Some (sm_inputs) , | | f (CurrentGcx :: new () , Proxy :: new ()) ,) }) . unwrap () . join () ; match r { Ok (v) => v , Err (e) => std :: panic :: resume_unwind (e) , } }) }
    };
}

run_in_thread_with_globals!()