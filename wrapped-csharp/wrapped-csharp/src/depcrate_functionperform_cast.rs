// Generated macro for perform_cast (function)
macro_rules! Depcrate_functionperform_cast {
() => {
// Module: crate::function
// Provides: {"perform_cast"}
// Dependencies: {}
fn perform_cast (op : & String , cast : & Bitcast) -> String { match cast { Bitcast :: I32ToF32 => format ! ("global::System.BitConverter.Int32BitsToSingle((int){op})") , Bitcast :: I64ToF32 => format ! ("global::System.BitConverter.Int32BitsToSingle((int){op})") , Bitcast :: F32ToI32 => format ! ("global::System.BitConverter.SingleToInt32Bits({op})") , Bitcast :: F32ToI64 => format ! ("global::System.BitConverter.SingleToInt32Bits({op})") , Bitcast :: I64ToF64 => format ! ("global::System.BitConverter.Int64BitsToDouble({op})") , Bitcast :: F64ToI64 => format ! ("global::System.BitConverter.DoubleToInt64Bits({op})") , Bitcast :: I32ToI64 => format ! ("(long) ({op})") , Bitcast :: I64ToI32 => format ! ("(int) ({op})") , Bitcast :: I64ToP64 => format ! ("{op}") , Bitcast :: P64ToI64 => format ! ("{op}") , Bitcast :: LToI64 | Bitcast :: PToP64 => format ! ("(long) ({op})") , Bitcast :: I64ToL | Bitcast :: P64ToP => format ! ("(int) ({op})") , Bitcast :: I32ToP | Bitcast :: PToI32 | Bitcast :: I32ToL | Bitcast :: LToI32 | Bitcast :: LToP | Bitcast :: PToL | Bitcast :: None => op . to_owned () , Bitcast :: Sequence (sequence) => { let [first , second] = & * * sequence ; perform_cast (& perform_cast (op , first) , second) } } }
};
}
