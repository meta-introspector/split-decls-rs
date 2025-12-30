// Generated macro for ConnectFut (enum)
macro_rules! Depcrate_connect_connectorConnectFut {
() => {
// Module: crate::connect::connector
// Provides: {"ConnectFut"}
// Dependencies: {}
# [doc = " Chains futures of resolve and connect steps."] pub (crate) enum ConnectFut < R : Host > { Resolve (< ResolverService as Service < ConnectInfo < R > > > :: Future) , Connect (< TcpConnectorService as Service < ConnectInfo < R > > > :: Future) , }
};
}
