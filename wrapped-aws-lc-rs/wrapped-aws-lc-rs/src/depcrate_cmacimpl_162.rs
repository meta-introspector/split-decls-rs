// Generated macro for impl_162 (impl)
macro_rules! Depcrate_cmacimpl_162 {
() => {
// Module: crate::cmac
// Provides: {"impl_162"}
// Dependencies: {}
impl core :: fmt :: Debug for Context { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> Result < () , core :: fmt :: Error > { f . debug_struct ("Context") . field ("algorithm" , & self . key . algorithm) . finish () } }
};
}
