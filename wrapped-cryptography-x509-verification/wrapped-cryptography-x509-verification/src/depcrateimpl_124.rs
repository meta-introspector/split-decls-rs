// Generated macro for impl_124 (impl)
macro_rules! Depcrateimpl_124 {
() => {
// Module: crate
// Provides: {"impl_124"}
// Dependencies: {}
impl < B : CryptoOps > From < DuplicateExtensionsError > for ValidationError < '_ , B > { fn from (value : DuplicateExtensionsError) -> Self { Self :: new (ValidationErrorKind :: ExtensionError { oid : value . 0 , reason : "duplicate extension" , }) } }
};
}
