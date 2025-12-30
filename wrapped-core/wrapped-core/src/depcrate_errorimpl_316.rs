// Generated macro for impl_316 (impl)
macro_rules! Depcrate_errorimpl_316 {
() => {
// Module: crate::error
// Provides: {"impl_316"}
// Dependencies: {}
impl From < Error > for syn :: Error { fn from (e : Error) -> Self { if e . len () == 1 { if let Some (span) = e . explicit_span () { syn :: Error :: new (span , e . kind) } else { syn :: Error :: new (e . span () , e) } } else { let mut syn_errors = e . flatten () . into_iter () . map (syn :: Error :: from) ; let mut error = syn_errors . next () . expect ("darling::Error can never be empty") ; for next_error in syn_errors { error . combine (next_error) ; } error } } }
};
}
