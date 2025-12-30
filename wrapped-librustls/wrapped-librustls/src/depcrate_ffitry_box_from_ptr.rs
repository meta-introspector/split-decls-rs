// Generated macro for try_box_from_ptr (macro)
macro_rules! Depcrate_ffitry_box_from_ptr {
() => {
// Module: crate::ffi
// Provides: {"try_box_from_ptr"}
// Dependencies: {}
# [doc = " If the provided pointer to a [`Castable`] is non-null, convert it to a reference to a `Box`"] # [doc = " over the underlying rust type using [`try_box_from`]. Otherwise, return [`rustls_result::NullParameter`],"] # [doc = " or an appropriate default (`false`, `0`, `NULL`) based on the context. See [`try_box_from`] for"] # [doc = " more information."] macro_rules ! try_box_from_ptr { ($ var : ident) => { match $ crate :: ffi :: try_box_from ($ var) { Some (c) => c , None => return $ crate :: panic :: NullParameterOrDefault :: value () , } } ; }
};
}
