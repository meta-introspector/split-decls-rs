// Generated macro for impl_29 (impl)
macro_rules! Depcrate_traitsimpl_29 {
() => {
// Module: crate::traits
// Provides: {"impl_29"}
// Dependencies: {}
impl < F : ConvertInto < T > , T > ConvertTryFrom < F > for T { type Error = Infallible ; fn convert_try_from (value : F) -> Result < T , Infallible > { Ok (value . convert_into ()) } }
};
}
