// Generated macro for MessageKind (enum)
macro_rules! Depcrate_client_non_io_typesMessageKind {
() => {
// Module: crate::client::non_io_types
// Provides: {"MessageKind"}
// Dependencies: {}
# [doc = " The kind of packet line to write when transforming a `RequestWriter` into an `ExtendedBufRead`."] # [doc = ""] # [doc = " Both the type and the trait have different implementations for blocking vs async I/O."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum MessageKind { # [doc = " A `flush` packet."] Flush , # [doc = " A V2 delimiter."] Delimiter , # [doc = " The end of a response."] ResponseEnd , # [doc = " The given text."] Text (& 'static [u8]) , }
};
}
