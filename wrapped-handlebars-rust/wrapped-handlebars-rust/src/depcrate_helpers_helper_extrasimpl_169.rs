// Generated macro for impl_169 (impl)
macro_rules! Depcrate_helpers_helper_extrasimpl_169 {
() => {
// Module: crate::helpers::helper_extras
// Provides: {"impl_169"}
// Dependencies: {}
impl crate :: HelperDef for ManyBoolHelper { fn call < 'reg : 'rc , 'rc > (& self , h : & crate :: Helper < 'rc > , r : & 'reg crate :: registry :: Registry < 'reg > , ctx : & 'rc crate :: Context , rc : & mut crate :: RenderContext < 'reg , 'rc > , out : & mut dyn crate :: Output ,) -> crate :: HelperResult { let value = self . call_inner (h , r , ctx , rc) ? ; let value = value . as_json () . as_bool () . unwrap_or (false) ; if ! (h . is_block ()) { return out . write (value . to_string () . as_str ()) . map_err (| e | crate :: RenderErrorReason :: Other (e . to_string ()) . into ()) ; } let tmpl = if value { h . template () } else { h . inverse () } ; match tmpl { Some (t) => t . render (r , ctx , rc , out) , None => Ok (()) , } } fn call_inner < 'reg : 'rc , 'rc > (& self , h : & crate :: Helper < 'rc > , _r : & 'reg crate :: Handlebars < 'reg > , _ : & 'rc crate :: Context , _ : & mut crate :: RenderContext < 'reg , 'rc > ,) -> std :: result :: Result < crate :: ScopedJson < 'rc > , crate :: RenderError > { let result = (self . op) (h . params ()) ; Ok (crate :: ScopedJson :: Derived (crate :: JsonValue :: from (result))) } }
};
}
