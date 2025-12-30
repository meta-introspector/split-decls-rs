// Generated macro for impl_552 (impl)
macro_rules! Depcrate_options_from_deriveimpl_552 {
() => {
// Module: crate::options::from_derive
// Provides: {"impl_552"}
// Dependencies: {}
impl < 'a > From < & 'a FdiOptions > for FromDeriveInputImpl < 'a > { fn from (v : & 'a FdiOptions) -> Self { FromDeriveInputImpl { base : (& v . base . container) . into () , attr_names : & v . base . attr_names , from_ident : v . base . from_ident , ident : v . base . ident . as_ref () , vis : v . vis . as_ref () , data : v . data . as_ref () , generics : v . generics . as_ref () , forward_attrs : v . base . as_forward_attrs () , supports : v . supports . as_ref () , } } }
};
}
