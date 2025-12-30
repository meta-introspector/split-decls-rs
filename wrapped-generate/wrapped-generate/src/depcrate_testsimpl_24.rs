// Generated macro for impl_24 (impl)
macro_rules! Depcrate_testsimpl_24 {
() => {
// Module: crate::tests
// Provides: {"impl_24"}
// Dependencies: {}
impl fmt :: Display for UIntTest { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . b { Some (b) => write ! (f , "
#[test]
#[allow(non_snake_case)]
fn test_{a}_{op}_{b}() {{
    type A = {gen_a};
    type B = {gen_b};
    type U{r} = {result};

    #[allow(non_camel_case_types)]
    type U{a}{op}U{b} = <<A as {op}<B>>::Output as Same<U{r}>>::Output;

    assert_eq!(<U{a}{op}U{b} as Unsigned>::to_u64(), <U{r} as Unsigned>::to_u64());
}}" , gen_a = gen_uint (self . a) , gen_b = gen_uint (b) , r = self . r , result = gen_uint (self . r) , a = self . a , b = b , op = self . op) , None => write ! (f , "
#[test]
#[allow(non_snake_case)]
fn test_{a}_{op}() {{
    type A = {gen_a};
    type U{r} = {result};

    #[allow(non_camel_case_types)]
    type {op}U{a} = <<A as {op}>::Output as Same<U{r}>>::Output;
    assert_eq!(<{op}U{a} as Unsigned>::to_u64(), <U{r} as Unsigned>::to_u64());
}}" , gen_a = gen_uint (self . a) , r = self . r , result = gen_uint (self . r) , a = self . a , op = self . op) , } } }
};
}
