// Generated macro for impl_get_feature (macro)
macro_rules! Depcrate_cpuimpl_get_feature {
() => {
// Module: crate::cpu
// Provides: {"impl_get_feature"}
// Dependencies: {}
macro_rules ! impl_get_feature { { $ ($ (# [$ meta : meta]) * $ Name : ident ,) + } => { $ ($ (# [$ meta]) * # [derive (Clone , Copy)] pub (crate) struct $ Name (crate :: cpu :: Features) ; $ (# [$ meta]) * impl $ Name { const fn mask () -> u32 { 1 << (Shift ::$ Name as u32) } } $ (# [$ meta]) * impl crate :: cpu :: GetFeature <$ Name > for super :: features :: Values { # [inline (always)] fn get_feature (& self) -> Option <$ Name > { const MASK : u32 = $ Name :: mask () ; const STATICALLY_DETECTED : bool = (crate :: cpu :: CAPS_STATIC & MASK) == MASK ; if STATICALLY_DETECTED { return Some ($ Name (self . cpu ())) ; } if (self . values () & MASK) == MASK { Some ($ Name (self . cpu ())) } else { None } } }) + # [repr (u32)] enum Shift { $ ($ (# [$ meta]) * $ Name ,) + # [cfg (target_arch = "x86_64")] IntelCpu , Initialized , } impl Shift { const INITIALIZED_MASK : core :: num :: NonZeroU32 = $ crate :: polyfill :: unwrap_const (core :: num :: NonZeroU32 :: new (1 << (Self :: Initialized as u32))) ; } } }
};
}
