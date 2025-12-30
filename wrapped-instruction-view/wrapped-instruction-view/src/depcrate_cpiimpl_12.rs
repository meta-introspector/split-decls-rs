// Generated macro for impl_12 (impl)
macro_rules! Depcrate_cpiimpl_12 {
() => {
// Module: crate::cpi
// Provides: {"impl_12"}
// Dependencies: {}
impl < 'a , const SIZE : usize > From < & 'a [u8 ; SIZE] > for Seed < 'a > { fn from (value : & 'a [u8 ; SIZE]) -> Self { Self { seed : value . as_ptr () , len : value . len () as u64 , _bytes : PhantomData :: < & [u8] > , } } }
};
}
