// Generated macro for try_clone_arc (macro)
macro_rules! Depcrate_ffitry_clone_arc {
() => {
// Module: crate::ffi
// Provides: {"try_clone_arc"}
// Dependencies: {}
# [doc = " If the provided pointer to a [`Castable`] is non-null, convert it to a reference to an `Arc` over"] # [doc = " the underlying rust type using [`clone_arc`]. Otherwise, return"] # [doc = " [`rustls_result::NullParameter`], or an appropriate default (`false`, `0`, `NULL`) based on the"] # [doc = " context. See [`clone_arc`] for more information."] macro_rules ! try_clone_arc { ($ var : ident) => { match $ crate :: ffi :: clone_arc ($ var) { Some (c) => c , None => return $ crate :: panic :: NullParameterOrDefault :: value () , } } ; }
};
}
