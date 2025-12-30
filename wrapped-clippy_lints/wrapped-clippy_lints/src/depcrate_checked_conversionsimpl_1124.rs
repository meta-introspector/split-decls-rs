// Generated macro for impl_1124 (impl)
macro_rules! Depcrate_checked_conversionsimpl_1124 {
() => {
// Module: crate::checked_conversions
// Provides: {"impl_1124"}
// Dependencies: {}
impl ConversionType { # [doc = " Creates a conversion type if the type is allowed & conversion is valid"] # [must_use] fn try_new (from : Symbol , to : Symbol) -> Option < Self > { if UINTS . contains (& from) { Some (Self :: FromUnsigned) } else if SINTS . contains (& from) { if UINTS . contains (& to) { Some (Self :: SignedToUnsigned) } else if SINTS . contains (& to) { Some (Self :: SignedToSigned) } else { None } } else { None } } }
};
}
