// Generated macro for impl_156 (impl)
macro_rules! Depcrate_helpers_helper_extrasimpl_156 {
() => {
// Module: crate::helpers::helper_extras
// Provides: {"impl_156"}
// Dependencies: {}
impl crate :: HelperDef for BinaryBoolHelper { fn call < 'reg : 'rc , 'rc > (& self , h : & crate :: Helper < 'rc > , r : & 'reg crate :: registry :: Registry < 'reg > , ctx : & 'rc crate :: Context , rc : & mut crate :: RenderContext < 'reg , 'rc > , out : & mut dyn crate :: Output ,) -> crate :: HelperResult { let value = self . call_inner (h , r , ctx , rc) ? ; let value = value . as_json () . as_bool () . unwrap_or (false) ; if ! (h . is_block ()) { return out . write (value . to_string () . as_str ()) . map_err (| e | crate :: RenderErrorReason :: Other (e . to_string ()) . into ()) ; } let tmpl = if value { h . template () } else { h . inverse () } ; match tmpl { Some (t) => t . render (r , ctx , rc , out) , None => Ok (()) , } } fn call_inner < 'reg : 'rc , 'rc > (& self , h : & crate :: Helper < 'rc > , r : & 'reg crate :: registry :: Registry < 'reg > , _ctx : & 'rc crate :: Context , _rc : & mut crate :: RenderContext < 'reg , 'rc > ,) -> Result < crate :: ScopedJson < 'rc > , crate :: RenderError > { let x = h . param (0) . and_then (| it | { if r . strict_mode () && it . is_value_missing () { None } else { Some (it . value ()) } }) . ok_or_else (| | crate :: RenderErrorReason :: ParamNotFoundForIndex (self . name , 0)) ? ; let y = h . param (1) . and_then (| it | { if r . strict_mode () && it . is_value_missing () { None } else { Some (it . value ()) } }) . ok_or_else (| | crate :: RenderErrorReason :: ParamNotFoundForIndex (self . name , 1)) ? ; Ok (crate :: ScopedJson :: Derived (Json :: Bool ((self . op) (x , y)))) } }
};
}
