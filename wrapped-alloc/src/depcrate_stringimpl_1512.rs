// Generated macro for impl_1512 (impl)
macro_rules! Depcrate_stringimpl_1512 {
() => {
// Module: crate::string
// Provides: {"impl_1512"}
// Dependencies: {}
impl < 'a > Drain < 'a > { # [doc = " Returns the remaining (sub)string of this iterator as a slice."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let mut s = String::from(\"abc\");"] # [doc = " let mut drain = s.drain(..);"] # [doc = " assert_eq!(drain.as_str(), \"abc\");"] # [doc = " let _ = drain.next().unwrap();"] # [doc = " assert_eq!(drain.as_str(), \"bc\");"] # [doc = " ```"] # [must_use] # [stable (feature = "string_drain_as_str" , since = "1.55.0")] pub fn as_str (& self) -> & str { self . iter . as_str () } }
};
}
