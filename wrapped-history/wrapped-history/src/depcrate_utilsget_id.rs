// Generated macro for get_id (function)
macro_rules! Depcrate_utilsget_id {
() => {
// Module: crate::utils
// Provides: {"get_id"}
// Dependencies: {}
# [cfg (all (target_arch = "wasm32" , not (target_os = "wasi")))] pub (crate) fn get_id () -> u32 { static ID_CTR : AtomicU32 = AtomicU32 :: new (0) ; static INIT : std :: sync :: Once = std :: sync :: Once :: new () ; INIT . call_once (| | { let mut start : [u8 ; 4] = [0 ; 4] ; let _ = getrandom :: getrandom (& mut start) ; ID_CTR . store (u32 :: from_ne_bytes (start) , Ordering :: SeqCst) ; }) ; ID_CTR . fetch_add (1 , Ordering :: SeqCst) }
};
}
