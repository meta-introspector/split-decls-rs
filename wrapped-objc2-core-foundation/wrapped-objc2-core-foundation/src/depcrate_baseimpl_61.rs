// Generated macro for impl_61 (impl)
macro_rules! Depcrate_baseimpl_61 {
() => {
// Module: crate::base
// Provides: {"impl_61"}
// Dependencies: {}
impl fmt :: Debug for CFType { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { # [cfg (feature = "CFString")] { let desc = crate :: CFCopyDescription (Some (self)) . expect ("must have description") ; write ! (f , "{desc}") } # [cfg (not (feature = "CFString"))] { f . debug_struct ("<CoreFoundation type (enable CFString feature for more info)>") . finish_non_exhaustive () } } }
};
}
