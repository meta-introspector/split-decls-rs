// Generated macro for impl_62 (impl)
macro_rules! Depcrate_decode_estimpl_62 {
() => {
// Module: crate::decode::est
// Provides: {"impl_62"}
// Dependencies: {}
impl MarkerLen { fn size (& self) -> u8 { match self . marker { Marker :: Bin8 => 1 , Marker :: Bin16 => 2 , Marker :: Bin32 => 4 , Marker :: Ext8 => 1 , Marker :: Ext16 => 2 , Marker :: Ext32 => 4 , Marker :: Str8 => 1 , Marker :: Str16 => 2 , Marker :: Str32 => 4 , Marker :: Array16 => 2 , Marker :: Array32 => 4 , Marker :: Map16 => 2 , Marker :: Map32 => 4 , _ => unimplemented ! () , } } }
};
}
