// Generated macro for impl_231 (impl)
macro_rules! Depcrate_validatorsimpl_231 {
() => {
// Module: crate::validators
// Provides: {"impl_231"}
// Dependencies: {}
impl FromMeta for UuidVersionValidation { fn from_word () -> darling :: Result < Self > { Ok (UuidVersionValidation :: None) } fn from_value (value : & Lit) -> darling :: Result < Self > { Ok (UuidVersionValidation :: Value (value . clone ())) } }
};
}
