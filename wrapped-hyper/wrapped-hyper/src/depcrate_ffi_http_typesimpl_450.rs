// Generated macro for impl_450 (impl)
macro_rules! Depcrate_ffi_http_typesimpl_450 {
() => {
// Module: crate::ffi::http_types
// Provides: {"impl_450"}
// Dependencies: {}
# [cfg (feature = "client")] impl crate :: ext :: OnInformationalCallback for OnInformational { fn on_informational (& self , res : http :: Response < () >) { let res = res . map (| () | IncomingBody :: empty ()) ; let mut res = hyper_response :: wrap (res) ; (self . func) (self . data . 0 , & mut res) ; } }
};
}
