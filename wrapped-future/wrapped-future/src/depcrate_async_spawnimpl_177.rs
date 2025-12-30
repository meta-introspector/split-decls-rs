// Generated macro for impl_177 (impl)
macro_rules! Depcrate_async_spawnimpl_177 {
() => {
// Module: crate::async_spawn
// Provides: {"impl_177"}
// Dependencies: {}
impl < P : RuntimeType > IAsyncInfo_Impl for ActionWithProgress_Impl < P > { fn Id (& self) -> Result < u32 > { Ok (1) } fn Status (& self) -> Result < AsyncStatus > { Ok (self . 0 . status ()) } fn ErrorCode (& self) -> Result < HRESULT > { Ok (self . 0 . error_code ()) } fn Cancel (& self) -> Result < () > { Ok (()) } fn Close (& self) -> Result < () > { Ok (()) } }
};
}
