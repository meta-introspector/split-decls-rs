// Generated macro for perform_cast (function)
macro_rules! Depcrateperform_cast {
() => {
// Module: crate
// Provides: {"perform_cast"}
// Dependencies: {}
fn perform_cast (op : & str , cast : & Bitcast) -> String { match cast { Bitcast :: I32ToF32 => { format ! ("({op}).reinterpret_as_float()") } Bitcast :: I64ToF32 => format ! ("({op}).to_int().reinterpret_as_float()") , Bitcast :: F32ToI32 => { format ! ("({op}).reinterpret_as_int()") } Bitcast :: F32ToI64 => format ! ("({op}).reinterpret_as_int().to_int64()") , Bitcast :: I64ToF64 => { format ! ("({op}).reinterpret_as_double()") } Bitcast :: F64ToI64 => { format ! ("({op}).reinterpret_as_int64()") } Bitcast :: LToI64 | Bitcast :: PToP64 | Bitcast :: I32ToI64 => format ! ("Int::to_int64({op})") , Bitcast :: I64ToL | Bitcast :: P64ToP | Bitcast :: I64ToI32 => format ! ("Int64::to_int({op})") , Bitcast :: I64ToP64 | Bitcast :: P64ToI64 | Bitcast :: I32ToP | Bitcast :: PToI32 | Bitcast :: I32ToL | Bitcast :: LToI32 | Bitcast :: LToP | Bitcast :: PToL | Bitcast :: None => op . to_owned () , Bitcast :: Sequence (sequence) => { let [first , second] = & * * sequence ; perform_cast (& perform_cast (op , first) , second) } } }
};
}
