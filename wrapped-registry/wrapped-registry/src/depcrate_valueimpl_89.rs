// Generated macro for impl_89 (impl)
macro_rules! Depcrate_valueimpl_89 {
() => {
// Module: crate::value
// Provides: {"impl_89"}
// Dependencies: {}
impl TryFrom < Value > for Vec < String > { type Error = Error ; fn try_from (from : Value) -> Result < Self > { match from . ty { Type :: MultiString => Ok (from . data . as_wide () . split (| c | * c == 0) . map (String :: from_utf16_lossy) . collect ()) , _ => Ok (vec ! [String :: try_from (from) ?]) , } } }
};
}
