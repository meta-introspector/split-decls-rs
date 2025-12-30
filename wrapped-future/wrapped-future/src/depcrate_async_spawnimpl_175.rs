// Generated macro for impl_175 (impl)
macro_rules! Depcrate_async_spawnimpl_175 {
() => {
// Module: crate::async_spawn
// Provides: {"impl_175"}
// Dependencies: {}
impl IAsyncInfo_Impl for Action_Impl { fn Id (& self) -> Result < u32 > { Ok (1) } fn Status (& self) -> Result < AsyncStatus > { Ok (self . 0 . status ()) } fn ErrorCode (& self) -> Result < HRESULT > { Ok (self . 0 . error_code ()) } fn Cancel (& self) -> Result < () > { Ok (()) } fn Close (& self) -> Result < () > { Ok (()) } }
};
}
