// Generated macro for impl_167 (impl)
macro_rules! Depcrate_async_spawnimpl_167 {
() => {
// Module: crate::async_spawn
// Provides: {"impl_167"}
// Dependencies: {}
impl < T : Async > State < T > { fn status (& self) -> AsyncStatus { match & self . result { None => AsyncStatus :: Started , Some (Ok (_)) => AsyncStatus :: Completed , Some (Err (_)) => AsyncStatus :: Error , } } fn error_code (& self) -> HRESULT { match & self . result { Some (Err (error)) => error . code () , _ => HRESULT (0) , } } fn get_results (& self) -> Result < T :: Output > { match & self . result { Some (result) => result . clone () , None => Err (Error :: from_hresult (HRESULT (0x8000000Eu32 as i32))) , } } }
};
}
