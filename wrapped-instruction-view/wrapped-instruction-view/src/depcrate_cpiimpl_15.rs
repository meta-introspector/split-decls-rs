// Generated macro for impl_15 (impl)
macro_rules! Depcrate_cpiimpl_15 {
() => {
// Module: crate::cpi
// Provides: {"impl_15"}
// Dependencies: {}
impl < 'a , 'b > From < & 'b [Seed < 'a >] > for Signer < 'a , 'b > { fn from (value : & 'b [Seed < 'a >]) -> Self { Self { seeds : value . as_ptr () , len : value . len () as u64 , _seeds : PhantomData :: < & 'b [Seed < 'a >] > , } } }
};
}
