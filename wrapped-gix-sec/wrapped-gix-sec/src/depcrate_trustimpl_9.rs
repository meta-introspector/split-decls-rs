// Generated macro for impl_9 (impl)
macro_rules! Depcrate_trustimpl_9 {
() => {
// Module: crate::trust
// Provides: {"impl_9"}
// Dependencies: {}
impl < T > Mapping < T > { # [doc = " Obtain the value for the given trust `level`."] pub fn by_level (& self , level : Trust) -> & T { match level { Trust :: Full => & self . full , Trust :: Reduced => & self . reduced , } } # [doc = " Obtain the value for the given `level` once."] pub fn into_value_by_level (self , level : Trust) -> T { match level { Trust :: Full => self . full , Trust :: Reduced => self . reduced , } } }
};
}
