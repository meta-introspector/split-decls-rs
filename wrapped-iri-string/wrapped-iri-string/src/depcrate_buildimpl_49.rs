// Generated macro for impl_49 (impl)
macro_rules! Depcrate_buildimpl_49 {
() => {
// Module: crate::build
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'a , S : Spec > private :: Sealed < 'a > for RiStr < S > { fn validate_builder (builder : Builder < 'a >) -> Result < Built < 'a , Self > , Error > { if builder . scheme . is_none () { return Err (Error :: new ()) ; } let path_is_absolute = validate_builder_for_iri_reference :: < S > (& builder) ? ; Ok (Built { builder , path_is_absolute , _ty_str : PhantomData , }) } }
};
}
