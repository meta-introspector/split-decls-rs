// Generated macro for impl_47 (impl)
macro_rules! Depcrate_buildimpl_47 {
() => {
// Module: crate::build
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'a , S : Spec > private :: Sealed < 'a > for RiReferenceStr < S > { fn validate_builder (builder : Builder < 'a >) -> Result < Built < 'a , Self > , Error > { let path_is_absolute = validate_builder_for_iri_reference :: < S > (& builder) ? ; Ok (Built { builder , path_is_absolute , _ty_str : PhantomData , }) } }
};
}
