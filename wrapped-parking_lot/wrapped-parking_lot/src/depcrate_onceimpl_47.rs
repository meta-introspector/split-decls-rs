// Generated macro for impl_47 (impl)
macro_rules! Depcrate_onceimpl_47 {
() => {
// Module: crate::once
// Provides: {"impl_47"}
// Dependencies: {}
impl OnceState { # [doc = " Returns whether the associated `Once` has been poisoned."] # [doc = ""] # [doc = " Once an initialization routine for a `Once` has panicked it will forever"] # [doc = " indicate to future forced initialization routines that it is poisoned."] # [inline] pub fn poisoned (self) -> bool { matches ! (self , OnceState :: Poisoned) } # [doc = " Returns whether the associated `Once` has successfully executed a"] # [doc = " closure."] # [inline] pub fn done (self) -> bool { matches ! (self , OnceState :: Done) } }
};
}
