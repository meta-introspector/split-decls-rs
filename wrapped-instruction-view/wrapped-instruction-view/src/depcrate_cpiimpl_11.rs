// Generated macro for impl_11 (impl)
macro_rules! Depcrate_cpiimpl_11 {
() => {
// Module: crate::cpi
// Provides: {"impl_11"}
// Dependencies: {}
impl < 'a > From < & 'a [u8] > for Seed < 'a > { fn from (value : & 'a [u8]) -> Self { Self { seed : value . as_ptr () , len : value . len () as u64 , _bytes : PhantomData :: < & [u8] > , } } }
};
}
