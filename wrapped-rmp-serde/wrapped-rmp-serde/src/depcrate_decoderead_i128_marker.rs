// Generated macro for read_i128_marker (function)
macro_rules! Depcrate_decoderead_i128_marker {
() => {
// Module: crate::decode
// Provides: {"read_i128_marker"}
// Dependencies: {}
# [inline (never)] fn read_i128_marker < 'de , R : ReadSlice < 'de > > (marker : Marker , rd : & mut R) -> Result < i128 , Error > { Ok (match marker { Marker :: FixPos (val) => val . into () , Marker :: FixNeg (val) => val . into () , Marker :: U8 => rd . read_data_u8 () ? . into () , Marker :: U16 => rd . read_data_u16 () ? . into () , Marker :: U32 => rd . read_data_u32 () ? . into () , Marker :: U64 => rd . read_data_u64 () ? . into () , Marker :: I8 => rd . read_data_i8 () ? . into () , Marker :: I16 => rd . read_data_i16 () ? . into () , Marker :: I32 => rd . read_data_i32 () ? . into () , Marker :: I64 => rd . read_data_i64 () ? . into () , Marker :: Bin8 => { let len = read_u8 (& mut * rd) ? ; read_128_buf (rd , len) ? } , Marker :: FixArray (len) => read_128_buf (rd , len) ? , marker => return Err (Error :: TypeMismatch (marker)) , }) }
};
}
