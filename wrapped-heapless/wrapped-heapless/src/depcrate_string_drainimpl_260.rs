// Generated macro for impl_260 (impl)
macro_rules! Depcrate_string_drainimpl_260 {
() => {
// Module: crate::string::drain
// Provides: {"impl_260"}
// Dependencies: {}
impl < LenT : LenType > Drain < '_ , LenT > { # [doc = " Returns the remaining (sub)string of this iterator as a slice."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use heapless::String;"] # [doc = ""] # [doc = " let mut s = String::<8>::try_from(\"abc\").unwrap();"] # [doc = " let mut drain = s.drain(..);"] # [doc = " assert_eq!(drain.as_str(), \"abc\");"] # [doc = " let _ = drain.next().unwrap();"] # [doc = " assert_eq!(drain.as_str(), \"bc\");"] # [doc = " ```"] # [must_use] pub fn as_str (& self) -> & str { self . iter . as_str () } }
};
}
