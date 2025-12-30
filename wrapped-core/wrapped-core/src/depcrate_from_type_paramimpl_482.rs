// Generated macro for impl_482 (impl)
macro_rules! Depcrate_from_type_paramimpl_482 {
() => {
// Module: crate::from_type_param
// Provides: {"impl_482"}
// Dependencies: {}
impl FromTypeParam for Vec < syn :: Attribute > { fn from_type_param (type_param : & TypeParam) -> Result < Self > { Ok (type_param . attrs . clone ()) } }
};
}
