// Generated macro for impl_182 (impl)
macro_rules! Depcrate_helpers_helper_ifimpl_182 {
() => {
// Module: crate::helpers::helper_if
// Provides: {"impl_182"}
// Dependencies: {}
impl HelperDef for IfHelper { fn call < 'reg : 'rc , 'rc > (& self , h : & Helper < 'rc > , r : & 'reg Registry < 'reg > , ctx : & 'rc Context , rc : & mut RenderContext < 'reg , 'rc > , out : & mut dyn Output ,) -> HelperResult { let param = h . param (0) . ok_or (RenderErrorReason :: ParamNotFoundForIndex ("if" , 0)) ? ; let include_zero = h . hash_get ("includeZero") . and_then (| v | v . value () . as_bool ()) . unwrap_or (false) ; let mut value = param . value () . is_truthy (include_zero) ; if ! self . positive { value = ! value ; } let tmpl = if value { h . template () } else { h . inverse () } ; match tmpl { Some (t) => t . render (r , ctx , rc , out) , None => Ok (()) , } } }
};
}
