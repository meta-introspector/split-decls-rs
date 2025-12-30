// Generated macro for as_cbs (function)
macro_rules! Depcrateas_cbs {
() => {
// Module: crate
// Provides: {"as_cbs"}
// Dependencies: {}
# [cfg (feature = "mlalgs")] fn as_cbs (buf : & [u8]) -> bssl_sys :: CBS { bssl_sys :: CBS { data : buf . as_ffi_ptr () , len : buf . len () , } }
};
}
