// Generated macro for impl_25 (impl)
macro_rules! Depcrate_inoutimpl_25 {
() => {
// Module: crate::inout
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'inp , 'out , T , N : ArraySize > InOut < 'inp , 'out , Array < T , N > > { # [doc = " Returns `InOut` for the given position."] # [doc = ""] # [doc = " # Panics"] # [doc = " If `pos` greater or equal to array length."] # [inline (always)] pub fn get (& mut self , pos : usize) -> InOut < '_ , '_ , T > { assert ! (pos < N :: USIZE) ; unsafe { InOut { in_ptr : (self . in_ptr as * const T) . add (pos) , out_ptr : (self . out_ptr as * mut T) . add (pos) , _pd : PhantomData , } } } # [doc = " Convert `InOut` array to `InOutBuf`."] # [inline (always)] pub fn into_buf (self) -> InOutBuf < 'inp , 'out , T > { InOutBuf { in_ptr : self . in_ptr as * const T , out_ptr : self . out_ptr as * mut T , len : N :: USIZE , _pd : PhantomData , } } }
};
}
