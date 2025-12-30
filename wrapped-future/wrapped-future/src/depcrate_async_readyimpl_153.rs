// Generated macro for impl_153 (impl)
macro_rules! Depcrate_async_readyimpl_153 {
() => {
// Module: crate::async_ready
// Provides: {"impl_153"}
// Dependencies: {}
impl < P : RuntimeType > IAsyncInfo_Impl for ReadyActionWithProgress_Impl < P > { fn Id (& self) -> Result < u32 > { Ok (1) } fn Status (& self) -> Result < AsyncStatus > { Ok (self . 0 . status ()) } fn ErrorCode (& self) -> Result < HRESULT > { Ok (self . 0 . error_code ()) } fn Cancel (& self) -> Result < () > { Ok (()) } fn Close (& self) -> Result < () > { Ok (()) } }
};
}
