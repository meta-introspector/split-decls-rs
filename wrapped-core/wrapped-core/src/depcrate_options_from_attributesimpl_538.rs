// Generated macro for impl_538 (impl)
macro_rules! Depcrate_options_from_attributesimpl_538 {
() => {
// Module: crate::options::from_attributes
// Provides: {"impl_538"}
// Dependencies: {}
impl < 'a > From < & 'a FromAttributesOptions > for FromAttributesImpl < 'a > { fn from (v : & 'a FromAttributesOptions) -> Self { FromAttributesImpl { base : (& v . base . container) . into () , attr_names : & v . base . attr_names , forward_attrs : v . base . as_forward_attrs () , } } }
};
}
