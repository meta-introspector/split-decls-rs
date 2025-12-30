// Generated macro for impl_16 (impl)
macro_rules! Depcrate_cpiimpl_16 {
() => {
// Module: crate::cpi
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'a , 'b , const SIZE : usize > From < & 'b [Seed < 'a > ; SIZE] > for Signer < 'a , 'b > { fn from (value : & 'b [Seed < 'a > ; SIZE]) -> Self { Self { seeds : value . as_ptr () , len : value . len () as u64 , _seeds : PhantomData :: < & 'b [Seed < 'a >] > , } } }
};
}
