// Generated macro for impl_176 (impl)
macro_rules! Depcrate_async_spawnimpl_176 {
() => {
// Module: crate::async_spawn
// Provides: {"impl_176"}
// Dependencies: {}
impl < T : RuntimeType > IAsyncInfo_Impl for Operation_Impl < T > { fn Id (& self) -> Result < u32 > { Ok (1) } fn Status (& self) -> Result < AsyncStatus > { Ok (self . 0 . status ()) } fn ErrorCode (& self) -> Result < HRESULT > { Ok (self . 0 . error_code ()) } fn Cancel (& self) -> Result < () > { Ok (()) } fn Close (& self) -> Result < () > { Ok (()) } }
};
}
