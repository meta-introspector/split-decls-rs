// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl EcState { fn stuck (& mut self , current_delta : i32) -> bool { let delta2 = self . last_delta - current_delta ; let delta3 = delta2 - self . last_delta2 ; self . last_delta = current_delta ; self . last_delta2 = delta2 ; current_delta == 0 || delta2 == 0 || delta3 == 0 } }
};
}
