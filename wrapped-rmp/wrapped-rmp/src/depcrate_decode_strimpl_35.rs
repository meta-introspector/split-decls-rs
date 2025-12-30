// Generated macro for impl_35 (impl)
macro_rules! Depcrate_decode_strimpl_35 {
() => {
// Module: crate::decode::str
// Provides: {"impl_35"}
// Dependencies: {}
impl < E : RmpReadErr > Display for DecodeStringError < '_ , E > { # [cold] fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str ("error while decoding string") } }
};
}
