// Generated macro for impl_30 (impl)
macro_rules! Depcrate_testsimpl_30 {
() => {
// Module: crate::tests
// Provides: {"impl_30"}
// Dependencies: {}
impl fmt :: Display for IntUnaryTest { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "
#[test]
#[allow(non_snake_case)]
fn test_{sa}{a}_{op}() {{
    type A = {gen_a};
    type {sr}{r} = {result};

    #[allow(non_camel_case_types)]
    type {op}{sa}{a} = <<A as {op}>::Output as Same<{sr}{r}>>::Output;
    assert_eq!(<{op}{sa}{a} as Integer>::to_i64(), <{sr}{r} as Integer>::to_i64());
}}" , gen_a = gen_int (self . a) , r = self . r . abs () , sr = sign (self . r) , result = gen_int (self . r) , a = self . a . abs () , sa = sign (self . a) , op = self . op) } }
};
}
