// Generated macro for ToErrno (trait)
macro_rules! Depcrate_errnoToErrno {
() => {
// Module: crate::errno
// Provides: {"ToErrno"}
// Dependencies: {}
pub (crate) trait ToErrno { fn to_errno (& self) -> Option < i32 > { None } fn set_errno (self) -> Self where Self : Sized , { if let Some (errno) = self . to_errno () { cfg_if :: cfg_if ! { if # [cfg (any (feature = "common-os" , feature = "nostd" , target_arch = "riscv64"))] { let _ = errno ; } else { unsafe { * sys_errno_location () = errno ; } } } } self } }
};
}
