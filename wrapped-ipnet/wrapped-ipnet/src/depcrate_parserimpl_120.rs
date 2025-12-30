// Generated macro for impl_120 (impl)
macro_rules! Depcrate_parserimpl_120 {
() => {
// Module: crate::parser
// Provides: {"impl_120"}
// Dependencies: {}
impl FromStr for IpNet { type Err = AddrParseError ; fn from_str (s : & str) -> Result < IpNet , AddrParseError > { match Parser :: new (s) . read_till_eof (| p | p . read_ip_net ()) { Some (s) => Ok (s) , None => Err (AddrParseError (())) } } }
};
}
