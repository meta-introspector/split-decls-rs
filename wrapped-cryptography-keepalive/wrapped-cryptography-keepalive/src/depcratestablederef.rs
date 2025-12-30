// Generated macro for StableDeref (trait)
macro_rules! DepcrateStableDeref {
() => {
// Module: crate
// Provides: {"StableDeref"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = " Implementers of this trait must ensure that the value returned by"] # [doc = " `deref()` must remain valid, even if `self` is moved."] pub unsafe trait StableDeref : Deref { }
};
}
