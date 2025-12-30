// Generated macro for impl_227 (impl)
macro_rules! Depcrate_client_capabilitiesimpl_227 {
() => {
// Module: crate::client::capabilities
// Provides: {"impl_227"}
// Dependencies: {}
# [doc = " internal use"] # [cfg (any (feature = "blocking-client" , feature = "async-client"))] impl Capabilities { fn extract_protocol (capabilities_or_version : gix_packetline :: TextRef < '_ >) -> Result < Protocol , client :: Error > { let line = capabilities_or_version . as_bstr () ; let version = if line . starts_with_str ("version ") { if line . len () != "version X" . len () { return Err (client :: Error :: UnsupportedProtocolVersion (line . as_bstr () . into ())) ; } match line { line if line . ends_with_str ("1") => Protocol :: V1 , line if line . ends_with_str ("2") => Protocol :: V2 , _ => return Err (client :: Error :: UnsupportedProtocolVersion (line . as_bstr () . into ())) , } } else { Protocol :: V1 } ; Ok (version) } }
};
}
