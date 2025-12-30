// Generated macro for ffi_panic_boundary (macro)
macro_rules! Depcrate_panicffi_panic_boundary {
() => {
// Module: crate::panic
// Provides: {"ffi_panic_boundary"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! ffi_panic_boundary { ($ ($ tt : tt) *) => { match :: std :: panic :: catch_unwind (|| { $ ($ tt) * }) { Ok (ret) => ret , Err (_) => return $ crate :: panic :: PanicOrDefault :: value () , } } }
};
}
