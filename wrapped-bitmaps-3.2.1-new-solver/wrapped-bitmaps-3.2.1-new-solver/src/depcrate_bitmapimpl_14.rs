// Generated macro for impl_14 (impl)
macro_rules! Depcrate_bitmapimpl_14 {
() => {
// Module: crate::bitmap
// Provides: {"impl_14"}
// Dependencies: {}
impl < const SIZE : usize > PartialOrd for Bitmap < { SIZE } > where BitsImpl < { SIZE } > : Bits , < BitsImpl < { SIZE } > as Bits > :: Store : PartialOrd , { fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { self . as_value () . partial_cmp (other . as_value ()) } }
};
}
