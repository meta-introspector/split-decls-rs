// Generated macro for get_seed (function)
macro_rules! Depcrate_spanget_seed {
() => {
// Module: crate::span
// Provides: {"get_seed"}
// Dependencies: {}
fn get_seed () -> & 'static [u8] { & SEED . get_or_init (| | { if let Some (value) = option_env ! ("CONST_RANDOM_SEED") { Box :: new (value . as_bytes () . to_vec ()) } else { let mut value = [0u8 ; 32] ; getrandom :: getrandom (& mut value) . unwrap () ; Box :: new (value . to_vec ()) } }) [..] }
};
}
