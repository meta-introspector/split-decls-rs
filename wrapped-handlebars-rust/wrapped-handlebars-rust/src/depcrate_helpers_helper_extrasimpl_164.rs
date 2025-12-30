// Generated macro for impl_164 (impl)
macro_rules! Depcrate_helpers_helper_extrasimpl_164 {
() => {
// Module: crate::helpers::helper_extras
// Provides: {"impl_164"}
// Dependencies: {}
impl crate :: HelperDef for UnaryBoolHelper { fn call < 'reg : 'rc , 'rc > (& self , h : & crate :: Helper < 'rc > , r : & 'reg crate :: registry :: Registry < 'reg > , ctx : & 'rc crate :: Context , rc : & mut crate :: RenderContext < 'reg , 'rc > , out : & mut dyn crate :: Output ,) -> crate :: HelperResult { let value = self . call_inner (h , r , ctx , rc) ? ; let value = value . as_json () . as_bool () . unwrap_or (false) ; if ! (h . is_block ()) { return out . write (value . to_string () . as_str ()) . map_err (| e | crate :: RenderErrorReason :: Other (e . to_string ()) . into ()) ; } let tmpl = if value { h . template () } else { h . inverse () } ; match tmpl { Some (t) => t . render (r , ctx , rc , out) , None => Ok (()) , } } fn call_inner < 'reg : 'rc , 'rc > (& self , h : & crate :: Helper < 'rc > , r : & 'reg crate :: Handlebars < 'reg > , _ : & 'rc crate :: Context , _ : & mut crate :: RenderContext < 'reg , 'rc > ,) -> std :: result :: Result < crate :: ScopedJson < 'rc > , crate :: RenderError > { let arg = h . param (0) . and_then (| it | { if r . strict_mode () && it . is_value_missing () { None } else { Some (it . value ()) } }) . ok_or_else (| | crate :: RenderErrorReason :: ParamNotFoundForIndex (self . name , 0)) ? ; let result = (self . op) (arg) ; Ok (crate :: ScopedJson :: Derived (crate :: JsonValue :: from (result))) } }
};
}
