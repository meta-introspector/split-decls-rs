// Generated macro for Arg (enum)
macro_rules! DepcrateArg {
() => {
// Module: crate
// Provides: {"Arg"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq)] enum Arg < 't > { # [doc = " Bool"] Bool (bool) , F32 (f32) , F64 (f64) , # [doc = " U8, U16, U32, U64, U128"] Uxx (u128) , # [doc = " I8, I16, I32, I64, I128"] Ixx (i128) , # [doc = " Str"] Str (String) , # [doc = " Interned string"] IStr (& 't str) , # [doc = " Format"] Format { format : & 't str , args : Vec < Arg < 't > > , } , FormatSlice { elements : Vec < FormatSliceElement < 't > > , } , FormatSequence { args : Vec < Arg < 't > > , } , # [doc = " Slice or Array of bytes."] Slice (Vec < u8 >) , # [doc = " Char"] Char (char) , # [doc = " `fmt::Debug` / `fmt::Display` formatted on-target."] Preformatted (String) , }
};
}
