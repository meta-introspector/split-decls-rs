// Generated macro for Type (enum)
macro_rules! Depcrate_typesType {
() => {
// Module: crate::types
// Provides: {"Type"}
// Dependencies: {}
# [derive (Clone , Debug , Default , Eq , PartialEq)] pub enum Type { BitField (Range < u8 >) , Bool , # [doc = " A single Unicode character"] Char , Debug , Display , FormatSequence , F32 , F64 , # [doc = " `{=?}` OR `{}`"] # [default] Format , FormatArray (usize) , # [doc = " `{=[?]}`"] FormatSlice , I8 , I16 , I32 , I64 , I128 , Isize , # [doc = " Interned string index."] IStr , # [doc = " String slice (i.e. passed directly; not as interned string indices)."] Str , U8 , U16 , U32 , U64 , U128 , Usize , # [doc = " Byte slice `{=[u8]}`."] U8Slice , U8Array (usize) , }
};
}
