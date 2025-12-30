// Generated macro for impl_1027 (impl)
macro_rules! Depcrate_tz_posiximpl_1027 {
() => {
// Module: crate::tz::posix
// Provides: {"impl_1027"}
// Dependencies: {}
# [cfg (feature = "tz-system")] impl core :: fmt :: Display for PosixTzEnv { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { match * self { PosixTzEnv :: Rule (ref tz) => write ! (f , "{tz}") , PosixTzEnv :: Implementation (ref imp) => write ! (f , ":{imp}") , } } }
};
}
