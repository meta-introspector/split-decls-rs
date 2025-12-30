// Generated macro for impl_from (macro)
macro_rules! Depcrate_redactionimpl_from {
() => {
// Module: crate::redaction
// Provides: {"impl_from"}
// Dependencies: {}
macro_rules ! impl_from { ($ ty : ty) => { impl From <$ ty > for Redaction { fn from (value : $ ty) -> Redaction { Redaction :: Static (Content :: from (value)) } } } ; }
};
}
