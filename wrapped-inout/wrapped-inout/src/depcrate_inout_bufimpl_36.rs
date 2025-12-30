// Generated macro for impl_36 (impl)
macro_rules! Depcrate_inout_bufimpl_36 {
() => {
// Module: crate::inout_buf
// Provides: {"impl_36"}
// Dependencies: {}
impl < 'inp , 'out , T > IntoIterator for InOutBuf < 'inp , 'out , T > { type Item = InOut < 'inp , 'out , T > ; type IntoIter = InOutBufIter < 'inp , 'out , T > ; # [inline (always)] fn into_iter (self) -> Self :: IntoIter { InOutBufIter { buf : self , pos : 0 } } }
};
}
