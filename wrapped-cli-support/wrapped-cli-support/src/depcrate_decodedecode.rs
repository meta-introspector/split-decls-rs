// Generated macro for Decode (trait)
macro_rules! Depcrate_decodeDecode {
() => {
// Module: crate::decode
// Provides: {"Decode"}
// Dependencies: {}
pub trait Decode < 'src > : Sized { fn decode (data : & mut & 'src [u8]) -> Self ; fn decode_all (mut data : & 'src [u8]) -> Self { let ret = Self :: decode (& mut data) ; assert ! (data . is_empty ()) ; ret } }
};
}
