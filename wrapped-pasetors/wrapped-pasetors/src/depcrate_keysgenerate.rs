// Generated macro for Generate (trait)
macro_rules! Depcrate_keysGenerate {
() => {
// Module: crate::keys
// Provides: {"Generate"}
// Dependencies: {}
# [doc = " A type `T` that can be generated for a given version `V`."] pub trait Generate < T , V : Version > { # [doc = " Generate `T`."] fn generate () -> Result < T , Error > ; }
};
}
