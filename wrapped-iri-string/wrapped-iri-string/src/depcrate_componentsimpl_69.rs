// Generated macro for impl_69 (impl)
macro_rules! Depcrate_componentsimpl_69 {
() => {
// Module: crate::components
// Provides: {"impl_69"}
// Dependencies: {}
impl < 'a , S : Spec > From < & 'a RiReferenceStr < S > > for RiReferenceComponents < 'a , S > { # [inline] fn from (s : & 'a RiReferenceStr < S >) -> Self { trusted_parser :: decompose_iri_reference (s) } }
};
}
