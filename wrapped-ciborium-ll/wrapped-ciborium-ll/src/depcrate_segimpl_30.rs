// Generated macro for impl_30 (impl)
macro_rules! Depcrate_segimpl_30 {
() => {
// Module: crate::seg
// Provides: {"impl_30"}
// Dependencies: {}
impl Parser for Bytes { type Item = [u8] ; type Error = core :: convert :: Infallible ; fn parse < 'a > (& mut self , bytes : & 'a mut [u8]) -> Result < & 'a [u8] , Self :: Error > { Ok (bytes) } }
};
}
