// Generated macro for impl_39 (impl)
macro_rules! Depcrate_inout_bufimpl_39 {
() => {
// Module: crate::inout_buf
// Provides: {"impl_39"}
// Dependencies: {}
impl < 'inp , 'out , T , N > TryInto < InOut < 'inp , 'out , Array < T , N > > > for InOutBuf < 'inp , 'out , T > where N : ArraySize , { type Error = IntoArrayError ; # [inline (always)] fn try_into (self) -> Result < InOut < 'inp , 'out , Array < T , N > > , Self :: Error > { if self . len () == N :: USIZE { Ok (InOut { in_ptr : self . in_ptr as * const _ , out_ptr : self . out_ptr as * mut _ , _pd : PhantomData , }) } else { Err (IntoArrayError) } } }
};
}
