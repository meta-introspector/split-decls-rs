// Generated macro for marker_to_len (function)
macro_rules! Depcrate_decodemarker_to_len {
() => {
// Module: crate::decode
// Provides: {"marker_to_len"}
// Dependencies: {}
pub fn marker_to_len < R : RmpRead > (rd : & mut R , marker : Marker) -> Result < u32 , ValueReadError < R :: Error > > { match marker { Marker :: FixMap (size) => Ok (u32 :: from (size)) , Marker :: Map16 => Ok (u32 :: from (rd . read_data_u16 () ?)) , Marker :: Map32 => Ok (rd . read_data_u32 () ?) , marker => Err (ValueReadError :: TypeMismatch (marker)) , } }
};
}
