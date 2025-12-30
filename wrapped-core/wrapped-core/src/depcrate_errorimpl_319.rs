// Generated macro for impl_319 (impl)
macro_rules! Depcrate_errorimpl_319 {
() => {
// Module: crate::error
// Provides: {"impl_319"}
// Dependencies: {}
impl IntoIterator for Error { type Item = Error ; type IntoIter = IntoIter ; fn into_iter (self) -> IntoIter { if let ErrorKind :: Multiple (errors) = self . kind { IntoIter { inner : IntoIterEnum :: Multiple (errors . into_iter ()) , } } else { IntoIter { inner : IntoIterEnum :: Single (iter :: once (self)) , } } } }
};
}
