// Generated macro for impl_732 (impl)
macro_rules! Depcrate_frameimpl_732 {
() => {
// Module: crate::frame
// Provides: {"impl_732"}
// Dependencies: {}
impl fmt :: Debug for Ack { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut ranges = "[" . to_string () ; let mut first = true ; for range in self . iter () { if ! first { ranges . push (',') ; } write ! (ranges , "{range:?}") . unwrap () ; first = false ; } ranges . push (']') ; f . debug_struct ("Ack") . field ("largest" , & self . largest) . field ("delay" , & self . delay) . field ("ecn" , & self . ecn) . field ("ranges" , & ranges) . finish () } }
};
}
