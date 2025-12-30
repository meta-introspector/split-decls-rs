// Generated macro for sealed (module)
macro_rules! Depcrate_client_legacy_connect_dnssealed {
() => {
// Module: crate::client::legacy::connect::dns
// Provides: {"sealed"}
// Dependencies: {}
mod sealed { use std :: future :: Future ; use std :: task :: { self , Poll } ; use super :: { Name , SocketAddr } ; use tower_service :: Service ; pub trait Resolve { type Addrs : Iterator < Item = SocketAddr > ; type Error : Into < Box < dyn std :: error :: Error + Send + Sync > > ; type Future : Future < Output = Result < Self :: Addrs , Self :: Error > > ; fn poll_ready (& mut self , cx : & mut task :: Context < '_ >) -> Poll < Result < () , Self :: Error > > ; fn resolve (& mut self , name : Name) -> Self :: Future ; } impl < S > Resolve for S where S : Service < Name > , S :: Response : Iterator < Item = SocketAddr > , S :: Error : Into < Box < dyn std :: error :: Error + Send + Sync > > , { type Addrs = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; fn poll_ready (& mut self , cx : & mut task :: Context < '_ >) -> Poll < Result < () , Self :: Error > > { Service :: poll_ready (self , cx) } fn resolve (& mut self , name : Name) -> Self :: Future { Service :: call (self , name) } } }
};
}
