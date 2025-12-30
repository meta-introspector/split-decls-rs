// Generated macro for impl_188 (impl)
macro_rules! Depcrate_connect_connectorimpl_188 {
() => {
// Module: crate::connect::connector
// Provides: {"impl_188"}
// Dependencies: {}
impl Connector { # [doc = " Constructs new connector factory with the given resolver."] pub fn new (resolver : Resolver) -> Self { Connector { resolver } } # [doc = " Build connector service."] pub fn service (& self) -> ConnectorService { ConnectorService { tcp : TcpConnector :: default () . service () , resolver : self . resolver . service () , } } }
};
}
