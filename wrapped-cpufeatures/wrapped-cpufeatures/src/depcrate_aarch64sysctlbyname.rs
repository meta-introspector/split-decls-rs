// Generated macro for sysctlbyname (function)
macro_rules! Depcrate_aarch64sysctlbyname {
() => {
// Module: crate::aarch64
// Provides: {"sysctlbyname"}
// Dependencies: {}
# [doc = " Apple helper function for calling `sysctlbyname`."] # [cfg (target_vendor = "apple")] pub unsafe fn sysctlbyname (name : & [u8]) -> bool { assert_eq ! (name . last () . cloned () , Some (0) , "name is not NUL terminated: {:?}" , name) ; let mut value : u32 = 0 ; let mut size = core :: mem :: size_of :: < u32 > () ; let rc = unsafe { libc :: sysctlbyname (name . as_ptr () as * const i8 , & mut value as * mut _ as * mut libc :: c_void , & mut size , core :: ptr :: null_mut () , 0 ,) } ; assert_eq ! (size , 4 , "unexpected sysctlbyname(3) result size") ; assert_eq ! (rc , 0 , "sysctlbyname returned error code: {}" , rc) ; value != 0 }
};
}
