// Generated macro for impl_650 (impl)
macro_rules! Depcrate_utilsimpl_650 {
() => {
// Module: crate::utils
// Provides: {"impl_650"}
// Dependencies: {}
impl < 'a > io :: Read for Fountain < 'a > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let available = & self . chunk [self . consumed ..] ; let len = buf . len () . min (available . len ()) ; let (portion , _) = available . split_at (len) ; buf . copy_from_slice (portion) ; Ok (len) } }
};
}
