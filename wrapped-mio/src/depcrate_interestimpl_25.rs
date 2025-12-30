// Generated macro for impl_25 (impl)
macro_rules! Depcrate_interestimpl_25 {
() => {
// Module: crate::interest
// Provides: {"impl_25"}
// Dependencies: {}
impl fmt :: Debug for Interest { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut one = false ; if self . is_readable () { if one { write ! (fmt , " | ") ? } write ! (fmt , "READABLE") ? ; one = true } if self . is_writable () { if one { write ! (fmt , " | ") ? } write ! (fmt , "WRITABLE") ? ; one = true } # [cfg (any (target_os = "dragonfly" , target_os = "freebsd" , target_os = "ios" , target_os = "macos" , target_os = "tvos" , target_os = "visionos" , target_os = "watchos" ,))] { if self . is_aio () { if one { write ! (fmt , " | ") ? } write ! (fmt , "AIO") ? ; one = true } } # [cfg (target_os = "freebsd")] { if self . is_lio () { if one { write ! (fmt , " | ") ? } write ! (fmt , "LIO") ? ; one = true } } # [cfg (any (target_os = "linux" , target_os = "android"))] { if self . is_priority () { if one { write ! (fmt , " | ") ? } write ! (fmt , "PRIORITY") ? ; one = true } } debug_assert ! (one , "printing empty interests") ; Ok (()) } }
};
}
