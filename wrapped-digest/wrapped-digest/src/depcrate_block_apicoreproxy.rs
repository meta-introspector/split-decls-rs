// Generated macro for CoreProxy (trait)
macro_rules! Depcrate_block_apiCoreProxy {
() => {
// Module: crate::block_api
// Provides: {"CoreProxy"}
// Dependencies: {}
# [doc = " A proxy trait to the core block-level type."] pub trait CoreProxy { # [doc = " Core block-level type."] type Core : BufferKindUser ; # [doc = " Create `Self` from core and buffer."] fn compose (core : Self :: Core , buffer : Buffer < Self :: Core >) -> Self ; # [doc = " Decompose `self` into core and buffer."] fn decompose (self) -> (Self :: Core , Buffer < Self :: Core >) ; }
};
}
