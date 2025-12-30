// Generated macro for thread_local (macro)
macro_rules! Depcratethread_local {
() => {
// Module: crate
// Provides: {"thread_local"}
// Dependencies: {}
# [doc = " Mock version of `std::thread_local!`."] # [macro_export] macro_rules ! thread_local { () => { } ; ($ (# [$ attr : meta]) * $ vis : vis static $ name : ident : $ t : ty = $ init : expr ; $ ($ rest : tt) *) => ($ crate :: __thread_local_inner ! ($ (# [$ attr]) * $ vis $ name , $ t , $ init) ; $ crate :: thread_local ! ($ ($ rest) *) ;) ; ($ (# [$ attr : meta]) * $ vis : vis static $ name : ident : $ t : ty = $ init : expr) => ($ crate :: __thread_local_inner ! ($ (# [$ attr]) * $ vis $ name , $ t , $ init) ;) ; }
};
}
