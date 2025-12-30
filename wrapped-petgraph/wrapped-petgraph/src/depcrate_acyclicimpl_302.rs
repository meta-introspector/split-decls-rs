// Generated macro for impl_302 (impl)
macro_rules! Depcrate_acyclicimpl_302 {
() => {
// Module: crate::acyclic
// Provides: {"impl_302"}
// Dependencies: {}
impl < G : Default + Visitable > Default for Acyclic < G > { fn default () -> Self { let graph : G = Default :: default () ; let order_map = Default :: default () ; let discovered = RefCell :: new (FixedBitSet :: default ()) ; let finished = RefCell :: new (FixedBitSet :: default ()) ; Self { graph , order_map , discovered , finished , } } }
};
}
