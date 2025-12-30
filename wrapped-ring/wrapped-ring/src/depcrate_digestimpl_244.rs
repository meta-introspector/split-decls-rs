// Generated macro for impl_244 (impl)
macro_rules! Depcrate_digestimpl_244 {
() => {
// Module: crate::digest
// Provides: {"impl_244"}
// Dependencies: {}
impl Digest { pub (crate) fn compute_from (algorithm : & 'static Algorithm , data : & [u8] , cpu : cpu :: Features ,) -> Result < Self , InputTooLongError > { let mut ctx = Context :: new (algorithm) ; ctx . update (data) ; ctx . try_finish (cpu) } # [doc = " The algorithm that was used to calculate the digest value."] # [inline (always)] pub fn algorithm (& self) -> & 'static Algorithm { self . algorithm } }
};
}
