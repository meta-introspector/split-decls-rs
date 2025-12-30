// Generated macro for impl_38 (impl)
macro_rules! Depcrate_i2cimpl_38 {
() => {
// Module: crate::i2c
// Provides: {"impl_38"}
// Dependencies: {}
impl core :: fmt :: Display for NoAcknowledgeSource { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { Self :: Address => write ! (f , "The device did not acknowledge its address") , Self :: Data => write ! (f , "The device did not acknowledge the data") , Self :: Unknown => write ! (f , "The device did not acknowledge its address or the data") , } } }
};
}
