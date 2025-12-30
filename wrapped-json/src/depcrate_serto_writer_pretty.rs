// Generated macro for to_writer_pretty (function)
macro_rules! Depcrate_serto_writer_pretty {
() => {
// Module: crate::ser
// Provides: {"to_writer_pretty"}
// Dependencies: {}
# [doc = " Serialize the given data structure as pretty-printed JSON into the I/O"] # [doc = " stream."] # [doc = ""] # [doc = " Serialization guarantees it only feeds valid UTF-8 sequences to the writer."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Serialization can fail if `T`'s implementation of `Serialize` decides to"] # [doc = " fail, or if `T` contains a map with non-string keys."] # [inline] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] pub fn to_writer_pretty < W , T > (writer : W , value : & T) -> Result < () > where W : io :: Write , T : ? Sized + Serialize , { let mut ser = Serializer :: pretty (writer) ; value . serialize (& mut ser) }
};
}
