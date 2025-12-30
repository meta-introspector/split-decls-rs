// Generated macro for impl_595 (impl)
macro_rules! Depcrate_options_from_type_paramimpl_595 {
() => {
// Module: crate::options::from_type_param
// Provides: {"impl_595"}
// Dependencies: {}
impl < 'a > From < & 'a FromTypeParamOptions > for FromTypeParamImpl < 'a > { fn from (v : & 'a FromTypeParamOptions) -> Self { FromTypeParamImpl { base : (& v . base . container) . into () , ident : v . base . ident . as_ref () , bounds : v . bounds . as_ref () , default : v . default . as_ref () , attr_names : & v . base . attr_names , forward_attrs : v . base . as_forward_attrs () , from_ident : v . base . from_ident , } } }
};
}
