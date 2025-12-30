// Generated macro for impl_122 (impl)
macro_rules! Depcrate_parserimpl_122 {
() => {
// Module: crate::parser
// Provides: {"impl_122"}
// Dependencies: {}
impl FromStr for Ipv6Net { type Err = AddrParseError ; fn from_str (s : & str) -> Result < Ipv6Net , AddrParseError > { match Parser :: new (s) . read_till_eof (| p | p . read_ipv6_net ()) { Some (s) => Ok (s) , None => Err (AddrParseError (())) } } }
};
}
