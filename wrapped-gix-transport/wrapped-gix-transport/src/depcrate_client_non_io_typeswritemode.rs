// Generated macro for WriteMode (enum)
macro_rules! Depcrate_client_non_io_typesWriteMode {
() => {
// Module: crate::client::non_io_types
// Provides: {"WriteMode"}
// Dependencies: {}
# [doc = " Configure how a `RequestWriter` behaves when writing bytes."] # [derive (Default , PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum WriteMode { # [doc = " Each [write()][std::io::Write::write()] call writes the bytes verbatim as one or more packet lines."] # [doc = ""] # [doc = " This mode also indicates to the transport that it should try to stream data as it is unbounded. This mode is typically used"] # [doc = " for sending packs whose exact size is not necessarily known in advance."] Binary , # [doc = " Each [write()][std::io::Write::write()] call assumes text in the input, assures a trailing newline and writes it as single packet line."] # [doc = ""] # [doc = " This mode also indicates that the lines written fit into memory, hence the transport may chose to not stream it but to buffer it"] # [doc = " instead. This is relevant for some transports, like the one for HTTP."] # [default] OneLfTerminatedLinePerWriteCall , }
};
}
