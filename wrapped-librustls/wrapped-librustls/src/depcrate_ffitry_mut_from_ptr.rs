// Generated macro for try_mut_from_ptr (macro)
macro_rules! Depcrate_ffitry_mut_from_ptr {
() => {
// Module: crate::ffi
// Provides: {"try_mut_from_ptr"}
// Dependencies: {}
# [doc = " If the provided pointer to a [`Castable`] is non-null, convert it to a mutable reference using"] # [doc = " [`try_from_mut`]. Otherwise, return [`rustls_result::NullParameter`], or an appropriate default"] # [doc = " (`false`, `0`, `NULL`) based on the context. See [`try_from_mut`] for more information."] macro_rules ! try_mut_from_ptr { ($ var : ident) => { match $ crate :: ffi :: try_from_mut ($ var) { Some (c) => c , None => return $ crate :: panic :: NullParameterOrDefault :: value () , } } ; }
};
}
