// Generated macro for Config (trait)
macro_rules! Depcrate_engineConfig {
() => {
// Module: crate::engine
// Provides: {"Config"}
// Dependencies: {}
# [doc = " The minimal level of configuration that engines must support."] pub trait Config { # [doc = " Returns `true` if padding should be added after the encoded output."] # [doc = ""] # [doc = " Padding is added outside the engine's `encode()` since the engine may be used"] # [doc = " to encode only a chunk of the overall output, so it can't always know when"] # [doc = " the output is \"done\" and would therefore need padding (if configured)."] fn encode_padding (& self) -> bool ; }
};
}
