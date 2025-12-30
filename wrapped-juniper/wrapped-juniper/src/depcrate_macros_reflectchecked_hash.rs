// Generated macro for checked_hash (macro)
macro_rules! Depcrate_macros_reflectchecked_hash {
() => {
// Module: crate::macros::reflect
// Provides: {"checked_hash"}
// Dependencies: {}
# [doc = " Ensures that the given `$impl_ty` implements [`Field`] and returns a"] # [doc = " [`fnv1a128`] hash for it, otherwise panics with understandable message."] # [macro_export] macro_rules ! checked_hash { ($ field_name : expr , $ impl_ty : ty , $ scalar : ty $ (, $ prefix : expr) ? $ (,) ?) => { { let exists = $ crate :: macros :: reflect :: str_exists_in_arr ($ field_name , <$ impl_ty as $ crate :: macros :: reflect :: Fields <$ scalar >>:: NAMES ,) ; if exists { $ crate :: macros :: reflect :: fnv1a128 (FIELD_NAME) } else { const MSG : & str = $ crate :: const_concat ! ($ ($ prefix ,) ? "field `" , $ field_name , "` isn't implemented on `" , <$ impl_ty as $ crate :: macros :: reflect :: BaseType <$ scalar >>:: NAME , "`") ; :: core :: panic ! ("{}" , MSG) } } } ; }
};
}
