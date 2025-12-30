// Generated macro for impl_204 (impl)
macro_rules! Depcrate_visit_reversedimpl_204 {
() => {
// Module: crate::visit::reversed
// Provides: {"impl_204"}
// Dependencies: {}
impl < G : Visitable > Visitable for Reversed < G > { type Map = G :: Map ; fn visit_map (& self) -> G :: Map { self . 0 . visit_map () } fn reset_map (& self , map : & mut Self :: Map) { self . 0 . reset_map (map) ; } }
};
}
