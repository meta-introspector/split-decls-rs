// Generated macro for impl_25 (impl)
macro_rules! Depcrate_lineimpl_25 {
() => {
// Module: crate::line
// Provides: {"impl_25"}
// Dependencies: {}
impl FromStr for TimeSpecAndType { type Err = Error ; fn from_str (input : & str) -> Result < Self , Self :: Err > { if input == "-" { return Ok (TimeSpecAndType (TimeSpec :: Zero , TimeType :: Wall)) ; } else if input . chars () . all (| c | c == '-' || c . is_ascii_digit ()) { return Ok (TimeSpecAndType (TimeSpec :: from_str (input) ? , TimeType :: Wall)) ; } let (input , ty) = match input . chars () . last () . and_then (TimeType :: from_char) { Some (ty) => (& input [.. input . len () - 1] , Some (ty)) , None => (input , None) , } ; let spec = TimeSpec :: from_str (input) ? ; Ok (TimeSpecAndType (spec , ty . unwrap_or (TimeType :: Wall))) } }
};
}
