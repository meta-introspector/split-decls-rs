// Generated macro for impl_559 (impl)
macro_rules! Depcrate_session_diagnosticsimpl_559 {
() => {
// Module: crate::session_diagnostics
// Provides: {"impl_559"}
// Dependencies: {}
impl InvalidIssueStringCause { pub (crate) fn from_int_error_kind (span : Span , kind : & IntErrorKind) -> Option < Self > { match kind { IntErrorKind :: Empty => Some (Self :: Empty { span }) , IntErrorKind :: InvalidDigit => Some (Self :: InvalidDigit { span }) , IntErrorKind :: PosOverflow => Some (Self :: PosOverflow { span }) , IntErrorKind :: NegOverflow => Some (Self :: NegOverflow { span }) , IntErrorKind :: Zero => Some (Self :: MustNotBeZero { span }) , _ => None , } } }
};
}
