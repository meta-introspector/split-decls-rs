// Generated macro for from_read_ref (function)
macro_rules! Depcrate_decodefrom_read_ref {
() => {
// Module: crate::decode
// Provides: {"from_read_ref"}
// Dependencies: {}
# [inline] # [doc (hidden)] # [deprecated (note = "use from_slice")] pub fn from_read_ref < 'a , R , T > (rd : & 'a R) -> Result < T , Error > where R : AsRef < [u8] > + ? Sized , T : Deserialize < 'a > , { let mut de = Deserializer :: from_read_ref (rd) ; Deserialize :: deserialize (& mut de) }
};
}
