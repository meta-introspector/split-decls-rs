// Generated macro for ext_len (function)
macro_rules! Depcrate_decodeext_len {
() => {
// Module: crate::decode
// Provides: {"ext_len"}
// Dependencies: {}
fn ext_len < R : Read > (rd : & mut R , marker : Marker) -> Result < u32 , Error > { Ok (match marker { Marker :: FixExt1 => 1 , Marker :: FixExt2 => 2 , Marker :: FixExt4 => 4 , Marker :: FixExt8 => 8 , Marker :: FixExt16 => 16 , Marker :: Ext8 => u32 :: from (read_u8 (rd) ?) , Marker :: Ext16 => u32 :: from (read_u16 (rd) ?) , Marker :: Ext32 => read_u32 (rd) ? , _ => return Err (Error :: TypeMismatch (marker)) , }) }
};
}
