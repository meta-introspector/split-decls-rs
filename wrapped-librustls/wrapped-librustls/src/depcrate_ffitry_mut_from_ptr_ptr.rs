// Generated macro for try_mut_from_ptr_ptr (macro)
macro_rules! Depcrate_ffitry_mut_from_ptr_ptr {
() => {
// Module: crate::ffi
// Provides: {"try_mut_from_ptr_ptr"}
// Dependencies: {}
# [doc = " If the provided pointer to a pointer to a [`Castable`] is non-null, convert it to a mutable"] # [doc = " reference to a pointer using [`try_from_mut_mut`]. Otherwise, return"] # [doc = " [`rustls_result::NullParameter`], or an appropriate default (`false`, `0`, `NULL`) based on the"] # [doc = " context. See [`try_from_mut_mut`] for more information."] macro_rules ! try_mut_from_ptr_ptr { ($ var : ident) => { match $ crate :: ffi :: try_from_mut_mut ($ var) { Some (c) => c , None => return $ crate :: panic :: NullParameterOrDefault :: value () , } } ; }
};
}
