// Generated macro for impl_211 (impl)
macro_rules! Depcrate_helpers_helper_lookupimpl_211 {
() => {
// Module: crate::helpers::helper_lookup
// Provides: {"impl_211"}
// Dependencies: {}
impl HelperDef for LookupHelper { fn call_inner < 'reg : 'rc , 'rc > (& self , h : & Helper < 'rc > , r : & 'reg Registry < 'reg > , _ : & 'rc Context , _ : & mut RenderContext < 'reg , 'rc > ,) -> Result < ScopedJson < 'rc > , RenderError > { let collection_value = h . param (0) . ok_or (RenderErrorReason :: ParamNotFoundForIndex ("lookup" , 0)) ? ; let index = h . param (1) . ok_or (RenderErrorReason :: ParamNotFoundForIndex ("lookup" , 1)) ? ; let value = match * collection_value . value () { Json :: Array (ref v) => index . value () . as_u64 () . and_then (| u | v . get (u as usize)) , Json :: Object (ref m) => index . value () . as_str () . and_then (| k | m . get (k)) , _ => None , } ; if r . strict_mode () && value . is_none () { Err (RenderError :: strict_error (None)) } else { Ok (value . unwrap_or (& Json :: Null) . clone () . into ()) } } }
};
}
