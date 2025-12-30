// Generated macro for mmio (function)
macro_rules! Depcrate_envmmio {
() => {
// Module: crate::env
// Provides: {"mmio"}
// Dependencies: {}
# [doc = " Returns the configuration of all mmio devices"] # [allow (dead_code)] pub fn mmio () -> & 'static [String] { CLI . get () . unwrap () . mmio . as_slice () }
};
}
