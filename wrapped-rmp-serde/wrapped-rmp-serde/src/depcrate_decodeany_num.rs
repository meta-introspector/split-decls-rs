// Generated macro for any_num (function)
macro_rules! Depcrate_decodeany_num {
() => {
// Module: crate::decode
// Provides: {"any_num"}
// Dependencies: {}
# [inline (never)] fn any_num < 'de , R : ReadSlice < 'de > , V : Visitor < 'de > > (rd : & mut R , visitor : V , marker : Marker) -> Result < V :: Value , Error > { match marker { Marker :: Null => visitor . visit_unit () , Marker :: True | Marker :: False => visitor . visit_bool (marker == Marker :: True) , Marker :: FixPos (val) => visitor . visit_u8 (val) , Marker :: FixNeg (val) => visitor . visit_i8 (val) , Marker :: U8 => visitor . visit_u8 (rd . read_data_u8 () ?) , Marker :: U16 => visitor . visit_u16 (rd . read_data_u16 () ?) , Marker :: U32 => visitor . visit_u32 (rd . read_data_u32 () ?) , Marker :: U64 => visitor . visit_u64 (rd . read_data_u64 () ?) , Marker :: I8 => visitor . visit_i8 (rd . read_data_i8 () ?) , Marker :: I16 => visitor . visit_i16 (rd . read_data_i16 () ?) , Marker :: I32 => visitor . visit_i32 (rd . read_data_i32 () ?) , Marker :: I64 => visitor . visit_i64 (rd . read_data_i64 () ?) , Marker :: F32 => visitor . visit_f32 (rd . read_data_f32 () ?) , Marker :: F64 => visitor . visit_f64 (rd . read_data_f64 () ?) , other_marker => Err (Error :: TypeMismatch (other_marker)) , } }
};
}
