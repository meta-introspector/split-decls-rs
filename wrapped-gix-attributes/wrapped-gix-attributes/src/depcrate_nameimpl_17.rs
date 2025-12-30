// Generated macro for impl_17 (impl)
macro_rules! Depcrate_nameimpl_17 {
() => {
// Module: crate::name
// Provides: {"impl_17"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a BStr > for NameRef < 'a > { type Error = Error ; fn try_from (attr : & 'a BStr) -> Result < Self , Self :: Error > { fn attr_valid (attr : & BStr) -> bool { if attr . first () == Some (& b'-') { return false ; } attr . bytes () . all (| b | matches ! (b , b'-' | b'.' | b'_' | b'A' ..= b'Z' | b'a' ..= b'z' | b'0' ..= b'9')) } attr_valid (attr) . then (| | NameRef (KStringRef :: from_ref (attr . to_str () . expect ("no illformed utf8")))) . ok_or_else (| | Error { attribute : attr . into () }) } }
};
}
