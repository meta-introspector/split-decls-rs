// Generated macro for ConnectorService (struct)
macro_rules! Depcrate_connect_connectorConnectorService {
() => {
// Module: crate::connect::connector
// Provides: {"ConnectorService"}
// Dependencies: {}
# [doc = " Combined resolver and TCP connector service."] # [doc = ""] # [doc = " Service implementation receives connection information, resolves DNS if required, and returns"] # [doc = " a TCP stream."] # [derive (Clone , Default)] pub struct ConnectorService { tcp : TcpConnectorService , resolver : ResolverService , }
};
}
