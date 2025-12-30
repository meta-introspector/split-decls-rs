// Generated macro for impl_366 (impl)
macro_rules! Depcrate_inlay_hintsimpl_366 {
() => {
// Module: crate::inlay_hints
// Provides: {"impl_366"}
// Dependencies: {}
impl < T > LazyProperty < T > { pub fn computed (self) -> Option < T > { match self { LazyProperty :: Computed (it) => Some (it) , _ => None , } } pub fn is_lazy (& self) -> bool { matches ! (self , Self :: Lazy) } }
};
}
