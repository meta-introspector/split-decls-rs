// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl FxHasher { # [doc = " Creates a `fx` hasher with a given seed."] pub const fn with_seed (seed : usize) -> FxHasher { FxHasher { hash : seed } } # [doc = " Creates a default `fx` hasher."] pub const fn default () -> FxHasher { FxHasher { hash : 0 } } }
};
}
