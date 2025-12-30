// Generated macro for read_ext_meta (function)
macro_rules! Depcrate_decode_extread_ext_meta {
() => {
// Module: crate::decode::ext
// Provides: {"read_ext_meta"}
// Dependencies: {}
pub fn read_ext_meta < R : RmpRead > (rd : & mut R) -> Result < ExtMeta , ValueReadError < R :: Error > > { let size = match read_marker (rd) ? { Marker :: FixExt1 => 1 , Marker :: FixExt2 => 2 , Marker :: FixExt4 => 4 , Marker :: FixExt8 => 8 , Marker :: FixExt16 => 16 , Marker :: Ext8 => u32 :: from (rd . read_data_u8 () ?) , Marker :: Ext16 => u32 :: from (rd . read_data_u16 () ?) , Marker :: Ext32 => rd . read_data_u32 () ? , marker => return Err (ValueReadError :: TypeMismatch (marker)) , } ; let ty = rd . read_data_i8 () ? ; let meta = ExtMeta { typeid : ty , size } ; Ok (meta) }
};
}
