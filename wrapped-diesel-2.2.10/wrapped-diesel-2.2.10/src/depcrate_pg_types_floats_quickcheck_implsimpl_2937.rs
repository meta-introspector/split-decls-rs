// Generated macro for impl_2937 (impl)
macro_rules! Depcrate_pg_types_floats_quickcheck_implsimpl_2937 {
() => {
// Module: crate::pg::types::floats::quickcheck_impls
// Provides: {"impl_2937"}
// Dependencies: {}
impl Arbitrary for Digit { fn arbitrary (g : & mut Gen) -> Self { let mut n = - 1 ; while ! (0 .. 10_000) . contains (& n) { n = i16 :: arbitrary (g) ; } Digit (n) } }
};
}
