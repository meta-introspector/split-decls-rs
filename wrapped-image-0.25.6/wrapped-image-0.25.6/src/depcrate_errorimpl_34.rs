// Generated macro for impl_34 (impl)
macro_rules! Depcrate_errorimpl_34 {
() => {
// Module: crate::error
// Provides: {"impl_34"}
// Dependencies: {}
impl LimitError { # [doc = " Construct a generic `LimitError` directly from a corresponding kind."] # [must_use] pub fn from_kind (kind : LimitErrorKind) -> Self { LimitError { kind } } # [doc = " Returns the corresponding `LimitErrorKind` of the error."] # [must_use] pub fn kind (& self) -> LimitErrorKind { self . kind . clone () } }
};
}
