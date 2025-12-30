// Generated macro for impl_84 (impl)
macro_rules! Depcrate_errorimpl_84 {
() => {
// Module: crate::error
// Provides: {"impl_84"}
// Dependencies: {}
impl core :: fmt :: Debug for Error { fn fmt (& self , fmt : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let mut debug = fmt . debug_struct ("Error") ; debug . field ("code" , & self . code ()) . field ("message" , & self . message ()) . finish () } }
};
}
