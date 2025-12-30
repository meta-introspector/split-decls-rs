// Generated macro for impl_208 (impl)
macro_rules! Depcrate_encodeimpl_208 {
() => {
// Module: crate::encode
// Provides: {"impl_208"}
// Dependencies: {}
impl < E : RmpWriteErr > Display for ValueWriteError < E > { # [cold] fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str ("error while writing multi-byte MessagePack value") } }
};
}
