// Generated macro for impl_194 (impl)
macro_rules! Depcrate_connect_connectorimpl_194 {
() => {
// Module: crate::connect::connector
// Provides: {"impl_194"}
// Dependencies: {}
impl < R : Host > ConnectFut < R > { fn poll_connect (& mut self , cx : & mut Context < '_ > ,) -> Poll < Result < ConnectFutState < R > , ConnectError > > { match self { ConnectFut :: Resolve (ref mut fut) => { Pin :: new (fut) . poll (cx) . map_ok (ConnectFutState :: Resolved) } ConnectFut :: Connect (ref mut fut) => { Pin :: new (fut) . poll (cx) . map_ok (ConnectFutState :: Connected) } } } }
};
}
