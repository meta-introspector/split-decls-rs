// Generated macro for impl_565 (impl)
macro_rules! Depcrate_options_from_fieldimpl_565 {
() => {
// Module: crate::options::from_field
// Provides: {"impl_565"}
// Dependencies: {}
impl < 'a > From < & 'a FromFieldOptions > for FromFieldImpl < 'a > { fn from (v : & 'a FromFieldOptions) -> Self { FromFieldImpl { ident : v . base . ident . as_ref () , vis : v . vis . as_ref () , ty : v . ty . as_ref () , base : (& v . base . container) . into () , attr_names : & v . base . attr_names , forward_attrs : v . base . as_forward_attrs () , from_ident : v . base . from_ident , } } }
};
}
