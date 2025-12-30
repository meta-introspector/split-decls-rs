// Generated macro for impl_863 (impl)
macro_rules! Depcrate_strategy_justimpl_863 {
() => {
// Module: crate::strategy::just
// Provides: {"impl_863"}
// Dependencies: {}
impl < T , F : Fn () -> T > fmt :: Debug for LazyJust < T , F > { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt . debug_struct ("LazyJust") . field ("function" , & "<function>") . finish () } }
};
}
