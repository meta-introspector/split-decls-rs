// Generated macro for check_attr (function)
macro_rules! Depcrate_parsecheck_attr {
() => {
// Module: crate::parse
// Provides: {"check_attr"}
// Dependencies: {}
fn check_attr (attr : & BStr) -> Result < NameRef < '_ > , name :: Error > { fn attr_valid (attr : & BStr) -> bool { if attr . first () == Some (& b'-') { return false ; } attr . bytes () . all (| b | matches ! (b , b'-' | b'.' | b'_' | b'A' ..= b'Z' | b'a' ..= b'z' | b'0' ..= b'9')) } attr_valid (attr) . then (| | NameRef (KStringRef :: from_ref (attr . to_str () . expect ("no illformed utf8")))) . ok_or_else (| | name :: Error { attribute : attr . into () }) }
};
}
