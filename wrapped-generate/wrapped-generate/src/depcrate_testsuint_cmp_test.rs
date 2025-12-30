// Generated macro for uint_cmp_test (function)
macro_rules! Depcrate_testsuint_cmp_test {
() => {
// Module: crate::tests
// Provides: {"uint_cmp_test"}
// Dependencies: {}
fn uint_cmp_test (a : u64 , b : u64) -> String { format ! ("
#[test]
#[allow(non_snake_case)]
fn test_{a}_Cmp_{b}() {{
    type A = {gen_a};
    type B = {gen_b};

    #[allow(non_camel_case_types)]
    type U{a}CmpU{b} = <A as Cmp<B>>::Output;
    assert_eq!(<U{a}CmpU{b} as Ord>::to_ordering(), Ordering::{result:?});
}}" , a = a , b = b , gen_a = gen_uint (a) , gen_b = gen_uint (b) , result = a . cmp (& b)) }
};
}
