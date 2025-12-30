// Generated macro for Connector (struct)
macro_rules! Depcrate_connect_connectorConnector {
() => {
// Module: crate::connect::connector
// Provides: {"Connector"}
// Dependencies: {}
# [doc = " Combined resolver and TCP connector service factory."] # [doc = ""] # [doc = " Used to create [`ConnectorService`]s which receive connection information, resolve DNS if"] # [doc = " required, and return a TCP stream."] # [derive (Clone , Default)] pub struct Connector { resolver : Resolver , }
};
}
