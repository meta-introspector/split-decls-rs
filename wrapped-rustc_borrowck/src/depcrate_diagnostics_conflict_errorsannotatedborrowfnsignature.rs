// Generated macro for AnnotatedBorrowFnSignature (enum)
macro_rules! Depcrate_diagnostics_conflict_errorsAnnotatedBorrowFnSignature {
() => {
// Module: crate::diagnostics::conflict_errors
// Provides: {"AnnotatedBorrowFnSignature"}
// Dependencies: {}
# [derive (Debug)] enum AnnotatedBorrowFnSignature < 'tcx > { NamedFunction { arguments : Vec < (Ty < 'tcx > , Span) > , return_ty : Ty < 'tcx > , return_span : Span , } , AnonymousFunction { argument_ty : Ty < 'tcx > , argument_span : Span , return_ty : Ty < 'tcx > , return_span : Span , } , Closure { argument_ty : Ty < 'tcx > , argument_span : Span , } , }
};
}
