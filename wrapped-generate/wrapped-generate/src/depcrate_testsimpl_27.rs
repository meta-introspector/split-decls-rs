// Generated macro for impl_27 (impl)
macro_rules! Depcrate_testsimpl_27 {
() => {
// Module: crate::tests
// Provides: {"impl_27"}
// Dependencies: {}
impl fmt :: Display for IntBinaryTest { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "
#[test]
#[allow(non_snake_case)]
fn test_{sa}{a}_{op}_{sb}{b}() {{
    type A = {gen_a};
    type B = {gen_b};
    type {sr}{r} = {result};

    #[allow(non_camel_case_types)]
    type {sa}{a}{op}{sb}{b} = <<A as {op}<B>>::Output as Same<{sr}{r}>>::Output;

    assert_eq!(<{sa}{a}{op}{sb}{b} as Integer>::to_i64(), <{sr}{r} as Integer>::to_i64());
}}" , gen_a = gen_int (self . a) , gen_b = gen_int (self . b) , r = self . r . abs () , sr = sign (self . r) , result = gen_int (self . r) , a = self . a . abs () , b = self . b . abs () , sa = sign (self . a) , sb = sign (self . b) , op = self . op) } }
};
}
