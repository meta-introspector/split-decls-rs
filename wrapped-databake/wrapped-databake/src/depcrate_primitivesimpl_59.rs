// Generated macro for impl_59 (impl)
macro_rules! Depcrate_primitivesimpl_59 {
() => {
// Module: crate::primitives
// Provides: {"impl_59"}
// Dependencies: {}
impl < T > BakeSize for & T where T : BakeSize , { fn borrows_size (& self) -> usize { core :: mem :: size_of_val :: < T > (* self) + (* self) . borrows_size () } }
};
}
