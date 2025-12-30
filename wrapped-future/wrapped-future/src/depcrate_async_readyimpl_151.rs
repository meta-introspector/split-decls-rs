// Generated macro for impl_151 (impl)
macro_rules! Depcrate_async_readyimpl_151 {
() => {
// Module: crate::async_ready
// Provides: {"impl_151"}
// Dependencies: {}
impl IAsyncInfo_Impl for ReadyAction_Impl { fn Id (& self) -> Result < u32 > { Ok (1) } fn Status (& self) -> Result < AsyncStatus > { Ok (self . 0 . status ()) } fn ErrorCode (& self) -> Result < HRESULT > { Ok (self . 0 . error_code ()) } fn Cancel (& self) -> Result < () > { Ok (()) } fn Close (& self) -> Result < () > { Ok (()) } }
};
}
