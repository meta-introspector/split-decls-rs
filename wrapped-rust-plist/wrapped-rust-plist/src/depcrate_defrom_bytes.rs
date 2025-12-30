// Generated macro for from_bytes (function)
macro_rules! Depcrate_defrom_bytes {
() => {
// Module: crate::de
// Provides: {"from_bytes"}
// Dependencies: {}
# [doc = " Deserializes an instance of type `T` from a byte slice."] pub fn from_bytes < T : de :: DeserializeOwned > (bytes : & [u8]) -> Result < T , Error > { let cursor = Cursor :: new (bytes) ; from_reader (cursor) }
};
}
