macro_rules! require_inited {
    () => {
        fn require_inited () { if ! INIT . is_completed () { bug ! ("LLVM is not initialized") ; } }
    };
}

require_inited!();