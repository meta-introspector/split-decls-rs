// Generated macro for impl_2919 (impl)
macro_rules! Depcrate_infinite_iterimpl_2919 {
() => {
// Module: crate::infinite_iter
// Provides: {"impl_2919"}
// Dependencies: {}
impl Finiteness { # [must_use] fn and (self , b : Self) -> Self { match (self , b) { (Finite , _) | (_ , Finite) => Finite , (MaybeInfinite , _) | (_ , MaybeInfinite) => MaybeInfinite , _ => Infinite , } } # [must_use] fn or (self , b : Self) -> Self { match (self , b) { (Infinite , _) | (_ , Infinite) => Infinite , (MaybeInfinite , _) | (_ , MaybeInfinite) => MaybeInfinite , _ => Finite , } } }
};
}
