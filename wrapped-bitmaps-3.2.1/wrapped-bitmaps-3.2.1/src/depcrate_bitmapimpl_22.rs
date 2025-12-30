// Generated macro for impl_22 (impl)
macro_rules! Depcrate_bitmapimpl_22 {
() => {
// Module: crate::bitmap
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'a , const SIZE : usize > IntoIterator for & 'a Bitmap < { SIZE } > where BitsImpl < { SIZE } > : Bits , { type Item = usize ; type IntoIter = Iter < 'a , { SIZE } > ; fn into_iter (self) -> Self :: IntoIter { Iter { head : None , tail : Some (SIZE + 1) , data : self , } } }
};
}
