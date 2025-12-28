macro_rules! init {
    () => {
        pub (crate) fn init (sess : & Session) { unsafe { if ! llvm :: LLVMIsMultithreaded () . is_true () { bug ! ("LLVM compiled without support for threads") ; } INIT . call_once (| | { configure_llvm (sess) ; }) ; } }
    };
}

init!();