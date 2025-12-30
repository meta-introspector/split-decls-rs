// Generated macro for impl_41 (impl)
macro_rules! Depcrate_inout_bufimpl_41 {
() => {
// Module: crate::inout_buf
// Provides: {"impl_41"}
// Dependencies: {}
impl < 'inp , 'out , T > Iterator for InOutBufIter < 'inp , 'out , T > { type Item = InOut < 'inp , 'out , T > ; # [inline (always)] fn next (& mut self) -> Option < Self :: Item > { if self . buf . len () == self . pos { return None ; } let res = unsafe { InOut { in_ptr : self . buf . in_ptr . add (self . pos) , out_ptr : self . buf . out_ptr . add (self . pos) , _pd : PhantomData , } } ; self . pos += 1 ; Some (res) } }
};
}
