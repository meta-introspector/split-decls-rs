// Generated macro for from_reader_ascii (function)
macro_rules! Depcrate_defrom_reader_ascii {
() => {
// Module: crate::de
// Provides: {"from_reader_ascii"}
// Dependencies: {}
# [doc = " Deserializes an instance of type `T` from a byte stream containing an ASCII encoded plist."] pub fn from_reader_ascii < R : Read , T : de :: DeserializeOwned > (reader : R) -> Result < T , Error > { let reader = stream :: AsciiReader :: new (reader) ; from_stream (reader) }
};
}
