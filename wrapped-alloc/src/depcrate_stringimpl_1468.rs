// Generated macro for impl_1468 (impl)
macro_rules! Depcrate_stringimpl_1468 {
() => {
// Module: crate::string
// Provides: {"impl_1468"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl FromStr for String { type Err = core :: convert :: Infallible ; # [inline] fn from_str (s : & str) -> Result < String , Self :: Err > { Ok (String :: from (s)) } }
};
}
