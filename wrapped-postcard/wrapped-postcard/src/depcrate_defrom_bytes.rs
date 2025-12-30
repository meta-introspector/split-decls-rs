// Generated macro for from_bytes (function)
macro_rules! Depcrate_defrom_bytes {
() => {
// Module: crate::de
// Provides: {"from_bytes"}
// Dependencies: {}
# [doc = " Deserialize a message of type `T` from a byte slice. The unused portion (if any)"] # [doc = " of the byte slice is not returned."] pub fn from_bytes < 'a , T > (s : & 'a [u8]) -> Result < T > where T : Deserialize < 'a > , { let mut deserializer = Deserializer :: from_bytes (s) ; let t = T :: deserialize (& mut deserializer) ? ; Ok (t) }
};
}
