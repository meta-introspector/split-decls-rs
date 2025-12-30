// Generated macro for impl_861 (impl)
macro_rules! Depcrate_output_timeimpl_861 {
() => {
// Module: crate::output::time
// Provides: {"impl_861"}
// Dependencies: {}
impl TimeFormat { pub fn format (self , time : & DateTime < FixedOffset >) -> String { # [rustfmt :: skip] return match self { Self :: DefaultFormat => default (time) , Self :: ISOFormat => iso (time) , Self :: LongISO => long (time) , Self :: FullISO => full (time) , Self :: Relative => relative (time) , Self :: Custom { non_recent , recent } => custom (time , non_recent . as_str () , recent . as_deref ()) , } ; } }
};
}
