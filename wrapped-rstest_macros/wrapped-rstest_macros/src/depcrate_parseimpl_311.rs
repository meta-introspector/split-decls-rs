// Generated macro for impl_311 (impl)
macro_rules! Depcrate_parseimpl_311 {
() => {
// Module: crate::parse
// Provides: {"impl_311"}
// Dependencies: {}
impl ExcludedTraceAttributesFunctionExtractor { pub (crate) fn take (self) -> Result < Vec < Pat > , ErrorsVec > { self . 0 } fn update_error (& mut self , mut errors : ErrorsVec) { match & mut self . 0 { Ok (_) => self . 0 = Err (errors) , Err (err) => err . append (& mut errors) , } } fn update_excluded (& mut self , value : Pat) { if let Some (inner) = self . 0 . iter_mut () . next () { inner . push (value) ; } } }
};
}
