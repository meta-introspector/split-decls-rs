// Generated macro for HashStableContext (trait)
macro_rules! DepcrateHashStableContext {
() => {
// Module: crate
// Provides: {"HashStableContext"}
// Dependencies: {}
# [doc = " Requirements for a `StableHashingContext` to be used in this crate."] # [doc = " This is a hack to allow using the `HashStable_Generic` derive macro"] # [doc = " instead of implementing everything in `rustc_middle`."] pub trait HashStableContext : rustc_span :: HashStableContext { }
};
}
