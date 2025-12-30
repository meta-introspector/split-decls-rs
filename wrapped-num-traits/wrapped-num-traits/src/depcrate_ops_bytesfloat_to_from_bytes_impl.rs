// Generated macro for float_to_from_bytes_impl (macro)
macro_rules! Depcrate_ops_bytesfloat_to_from_bytes_impl {
() => {
// Module: crate::ops::bytes
// Provides: {"float_to_from_bytes_impl"}
// Dependencies: {}
macro_rules ! float_to_from_bytes_impl { ($ T : ty , $ L : expr) => { impl ToBytes for $ T { type Bytes = [u8 ; $ L] ; # [inline] fn to_be_bytes (& self) -> Self :: Bytes { <$ T >:: to_be_bytes (* self) } # [inline] fn to_le_bytes (& self) -> Self :: Bytes { <$ T >:: to_le_bytes (* self) } # [inline] fn to_ne_bytes (& self) -> Self :: Bytes { <$ T >:: to_ne_bytes (* self) } } impl FromBytes for $ T { type Bytes = [u8 ; $ L] ; # [inline] fn from_be_bytes (bytes : & Self :: Bytes) -> Self { <$ T >:: from_be_bytes (* bytes) } # [inline] fn from_le_bytes (bytes : & Self :: Bytes) -> Self { <$ T >:: from_le_bytes (* bytes) } # [inline] fn from_ne_bytes (bytes : & Self :: Bytes) -> Self { <$ T >:: from_ne_bytes (* bytes) } } } ; }
};
}
