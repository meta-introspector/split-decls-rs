// Generated macro for impl_88 (impl)
macro_rules! Depcrate_errorimpl_88 {
() => {
// Module: crate::error
// Provides: {"impl_88"}
// Dependencies: {}
impl core :: fmt :: Display for WindowsError { # [cfg (feature = "std")] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let error = std :: io :: Error :: from_raw_os_error (self . 0) ; core :: fmt :: Display :: fmt (& error , f) } # [cfg (not (feature = "std"))] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . write_fmt (format_args ! ("OS error {}" , self . 0)) } }
};
}
