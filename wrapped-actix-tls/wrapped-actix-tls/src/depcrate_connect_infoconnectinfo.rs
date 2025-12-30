// Generated macro for ConnectInfo (struct)
macro_rules! Depcrate_connect_infoConnectInfo {
() => {
// Module: crate::connect::info
// Provides: {"ConnectInfo"}
// Dependencies: {}
# [doc = " Connection request information."] # [doc = ""] # [doc = " May contain known/pre-resolved socket address(es) or a host that needs resolving with DNS."] # [derive (Debug , PartialEq , Eq , Hash)] pub struct ConnectInfo < R > { pub (crate) request : R , pub (crate) port : u16 , pub (crate) addr : ConnectAddrs , pub (crate) local_addr : Option < IpAddr > , }
};
}
