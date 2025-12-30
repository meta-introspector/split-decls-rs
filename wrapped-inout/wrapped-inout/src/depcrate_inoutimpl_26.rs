// Generated macro for impl_26 (impl)
macro_rules! Depcrate_inoutimpl_26 {
() => {
// Module: crate::inout
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'inp , 'out , T , N , M > From < InOut < 'inp , 'out , Array < T , Prod < N , M > > > > for Array < InOut < 'inp , 'out , Array < T , N > > , M > where N : ArraySize , M : ArraySize , N : Mul < M > , Prod < N , M > : ArraySize , { fn from (buf : InOut < 'inp , 'out , Array < T , Prod < N , M > > >) -> Self { let in_ptr : * const Array < T , N > = buf . in_ptr . cast () ; let out_ptr : * mut Array < T , N > = buf . out_ptr . cast () ; Array :: from_fn (| i | unsafe { InOut { in_ptr : in_ptr . add (i) , out_ptr : out_ptr . add (i) , _pd : PhantomData , } }) } }
};
}
