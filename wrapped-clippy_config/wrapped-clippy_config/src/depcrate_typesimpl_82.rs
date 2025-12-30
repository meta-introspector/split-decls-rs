// Generated macro for impl_82 (impl)
macro_rules! Depcrate_typesimpl_82 {
() => {
// Module: crate::types
// Provides: {"impl_82"}
// Dependencies: {}
impl < const REPLACEMENT_ALLOWED : bool > DisallowedPath < REPLACEMENT_ALLOWED > { pub fn path (& self) -> & str { & self . path } pub fn diag_amendment (& self , span : Span) -> impl FnOnce (& mut Diag < '_ , () >) { move | diag | { if let Some (replacement) = & self . replacement { diag . span_suggestion (span , self . reason . as_ref () . map_or_else (| | String :: from ("use") , Clone :: clone) , replacement , Applicability :: MachineApplicable ,) ; } else if let Some (reason) = & self . reason { diag . note (reason . clone ()) ; } } } pub fn span (& self) -> Span { self . span } pub fn set_span (& mut self , span : Span) { self . span = span ; } }
};
}
