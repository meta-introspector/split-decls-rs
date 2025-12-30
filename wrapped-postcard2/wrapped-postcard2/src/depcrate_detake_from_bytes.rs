// Generated macro for take_from_bytes (function)
macro_rules! Depcrate_detake_from_bytes {
() => {
// Module: crate::de
// Provides: {"take_from_bytes"}
// Dependencies: {}
# [doc = " Deserialize a message of type `T` from a byte slice. The unused portion (if any)"] # [doc = " of the byte slice is returned for further usage"] pub fn take_from_bytes < 'a , T > (s : & 'a [u8]) -> Result < (T , & 'a [u8]) > where T : Deserialize < 'a > , { let mut deserializer = Deserializer :: from_bytes (s) ; let t = T :: deserialize (& mut deserializer) ? ; Ok ((t , deserializer . finalize () ?)) }
};
}
