// Generated macro for impl_15 (impl)
macro_rules! Depcrate_bitmapimpl_15 {
() => {
// Module: crate::bitmap
// Provides: {"impl_15"}
// Dependencies: {}
impl < const SIZE : usize > Ord for Bitmap < { SIZE } > where BitsImpl < { SIZE } > : Bits , < BitsImpl < { SIZE } > as Bits > :: Store : Ord , { fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { self . as_value () . cmp (other . as_value ()) } }
};
}
