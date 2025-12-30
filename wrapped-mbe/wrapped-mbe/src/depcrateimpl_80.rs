// Generated macro for impl_80 (impl)
macro_rules! Depcrateimpl_80 {
() => {
// Module: crate
// Provides: {"impl_80"}
// Dependencies: {}
impl ExpandError { fn new (span : Span , kind : ExpandErrorKind) -> ExpandError { ExpandError { inner : Arc :: new ((span , kind)) } } fn binding_error (span : Span , e : impl Into < Box < str > >) -> ExpandError { ExpandError { inner : Arc :: new ((span , ExpandErrorKind :: BindingError (Box :: new (e . into ())))) } } }
};
}
