// Generated macro for BitPack (trait)
macro_rules! Depcrate_encodeBitPack {
() => {
// Module: crate::encode
// Provides: {"BitPack"}
// Dependencies: {}
# [doc = " `BitPack` represents range-encoding logic"] pub (crate) trait BitPack < A , B > { type PackedSize : ArraySize ; fn pack (& self) -> Array < u8 , Self :: PackedSize > ; fn unpack (enc : & Array < u8 , Self :: PackedSize >) -> Self ; }
};
}
