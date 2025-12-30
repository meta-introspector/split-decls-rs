// Generated macro for FenceKind (enum)
macro_rules! Depcrate_isa_x64_inst_argsFenceKind {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"FenceKind"}
// Dependencies: {}
# [doc = " An x64 memory fence kind."] # [derive (Clone)] # [allow (dead_code)] pub enum FenceKind { # [doc = " `mfence` instruction (\"Memory Fence\")"] MFence , # [doc = " `lfence` instruction (\"Load Fence\")"] LFence , # [doc = " `sfence` instruction (\"Store Fence\")"] SFence , }
};
}
