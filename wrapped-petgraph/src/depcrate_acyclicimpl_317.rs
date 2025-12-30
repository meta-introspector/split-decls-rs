// Generated macro for impl_317 (impl)
macro_rules! Depcrate_acyclicimpl_317 {
() => {
// Module: crate::acyclic
// Provides: {"impl_317"}
// Dependencies: {}
impl < G : Visitable > Visitable for Acyclic < G > { type Map = G :: Map ; fn visit_map (& self) -> Self :: Map { self . inner () . visit_map () } fn reset_map (& self , map : & mut Self :: Map) { self . inner () . reset_map (map) } }
};
}
