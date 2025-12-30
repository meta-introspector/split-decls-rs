// Generated macro for from_reader (function)
macro_rules! Depcrate_defrom_reader {
() => {
// Module: crate::de
// Provides: {"from_reader"}
// Dependencies: {}
# [doc = " Deserializes an instance of type `T` from a seekable byte stream containing a plist of any encoding."] pub fn from_reader < R : Read + Seek , T : de :: DeserializeOwned > (reader : R) -> Result < T , Error > { let reader = stream :: Reader :: new (reader) ; from_stream (reader) }
};
}
