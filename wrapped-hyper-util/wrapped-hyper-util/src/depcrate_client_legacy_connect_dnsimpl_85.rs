// Generated macro for impl_85 (impl)
macro_rules! Depcrate_client_legacy_connect_dnsimpl_85 {
() => {
// Module: crate::client::legacy::connect::dns
// Provides: {"impl_85"}
// Dependencies: {}
impl FromStr for Name { type Err = InvalidNameError ; fn from_str (host : & str) -> Result < Self , Self :: Err > { Ok (Name :: new (host . into ())) } }
};
}
