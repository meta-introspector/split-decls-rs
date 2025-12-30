// Generated macro for impl_121 (impl)
macro_rules! Depcrate_parserimpl_121 {
() => {
// Module: crate::parser
// Provides: {"impl_121"}
// Dependencies: {}
impl FromStr for Ipv4Net { type Err = AddrParseError ; fn from_str (s : & str) -> Result < Ipv4Net , AddrParseError > { match Parser :: new (s) . read_till_eof (| p | p . read_ipv4_net ()) { Some (s) => Ok (s) , None => Err (AddrParseError (())) } } }
};
}
