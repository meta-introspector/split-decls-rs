// Generated macro for int_cmp_test (function)
macro_rules! Depcrate_testsint_cmp_test {
() => {
// Module: crate::tests
// Provides: {"int_cmp_test"}
// Dependencies: {}
fn int_cmp_test (a : i64 , b : i64) -> String { format ! ("
#[test]
#[allow(non_snake_case)]
fn test_{sa}{a}_Cmp_{sb}{b}() {{
    type A = {gen_a};
    type B = {gen_b};

    #[allow(non_camel_case_types)]
    type {sa}{a}Cmp{sb}{b} = <A as Cmp<B>>::Output;
    assert_eq!(<{sa}{a}Cmp{sb}{b} as Ord>::to_ordering(), Ordering::{result:?});
}}" , a = a . abs () , b = b . abs () , sa = sign (a) , sb = sign (b) , gen_a = gen_int (a) , gen_b = gen_int (b) , result = a . cmp (& b)) }
};
}
