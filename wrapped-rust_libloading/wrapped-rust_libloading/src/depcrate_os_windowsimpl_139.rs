// Generated macro for impl_139 (impl)
macro_rules! Depcrate_os_windowsimpl_139 {
() => {
// Module: crate::os::windows
// Provides: {"impl_139"}
// Dependencies: {}
impl fmt :: Debug for Library { # [cfg (feature = "std")] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { unsafe { let mut buf = mem :: MaybeUninit :: < [mem :: MaybeUninit < u16 > ; 1024] > :: uninit () . assume_init () ; let len = GetModuleFileNameW (self . 0 , buf [..] . as_mut_ptr () . cast () , 1024) as usize ; if len == 0 { f . write_fmt (format_args ! ("Library@{:#x}" , self . 0)) } else { let string : std :: ffi :: OsString = std :: os :: windows :: ffi :: OsStringExt :: from_wide (& * (& buf [.. len] as * const [_] as * const [u16]) ,) ; f . write_fmt (format_args ! ("Library@{:#x} from {:?}" , self . 0 , string)) } } } # [cfg (not (feature = "std"))] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_fmt (format_args ! ("Library@{:#x}" , self . 0)) } }
};
}
