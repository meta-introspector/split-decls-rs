// Generated macro for impl_136 (impl)
macro_rules! Depcrate_rpc_statusimpl_136 {
() => {
// Module: crate::rpc_status
// Provides: {"impl_136"}
// Dependencies: {}
impl RPC_STATUS { # [doc = " Returns [`true`] if `self` is a success code."] # [inline] pub const fn is_ok (self) -> bool { self . 0 == 0 } # [doc = " Returns [`true`] if `self` is a failure code."] # [inline] pub const fn is_err (self) -> bool { ! self . is_ok () } # [doc = " Maps an RPC error code to an HRESULT value."] # [inline] pub const fn to_hresult (self) -> HRESULT { WIN32_ERROR (self . 0 as u32) . to_hresult () } # [doc = " Converts the [`RPC_STATUS`] to [`Result<()>`][Result<_>]."] # [inline] pub fn ok (self) -> Result < () > { self . to_hresult () . ok () } }
};
}
