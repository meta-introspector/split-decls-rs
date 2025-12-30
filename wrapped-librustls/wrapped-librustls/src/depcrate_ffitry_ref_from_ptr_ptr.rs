// Generated macro for try_ref_from_ptr_ptr (macro)
macro_rules! Depcrate_ffitry_ref_from_ptr_ptr {
() => {
// Module: crate::ffi
// Provides: {"try_ref_from_ptr_ptr"}
// Dependencies: {}
# [doc = " If the provided pointer to pointer to a [`Castable`] is non-null, convert it to a mutable"] # [doc = " reference to a pointer to the [`Castable`] using"] # [doc = " [`try_from_ptr`]. Otherwise, return [`rustls_result::NullParameter`], or an appropriate default"] # [doc = " (`false`, `0`, `NULL`) based on the context;"] # [doc = ""] # [doc = " See [`try_from_ptr`] for more information."] macro_rules ! try_ref_from_ptr_ptr { ($ var : ident) => { match $ crate :: ffi :: try_from_ptr ($ var) { Some (c) => c , None => return $ crate :: panic :: NullParameterOrDefault :: value () , } } ; }
};
}
