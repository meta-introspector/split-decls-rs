// Generated macro for Marker (enum)
macro_rules! Depcrate_markerMarker {
() => {
// Module: crate::marker
// Provides: {"Marker"}
// Dependencies: {}
# [doc = " Format markers."] # [derive (Clone , Copy , Debug , PartialEq)] # [repr (u8)] pub enum Marker { FixPos (u8) = 0x00 , FixMap (u8) = 0x80 , FixArray (u8) = 0x90 , FixStr (u8) = 0xa0 , Null = 0xc0 , # [doc = " Marked in MessagePack spec as never used."] Reserved , False , True , Bin8 , Bin16 , Bin32 , Ext8 , Ext16 , Ext32 , F32 , F64 , U8 , U16 , U32 , U64 , I8 , I16 , I32 , I64 , FixExt1 , FixExt2 , FixExt4 , FixExt8 , FixExt16 , Str8 , Str16 , Str32 , Array16 , Array32 , Map16 , Map32 , FixNeg (i8) = 0xe0 , }
};
}
