// Generated macro for impl_tuples (macro)
macro_rules! Depcrate_test_traitsimpl_tuples {
() => {
// Module: crate::test_traits
// Provides: {"impl_tuples"}
// Dependencies: {}
# [doc = " Implement `CheckOutput` for combinations of types."] macro_rules ! impl_tuples { ($ (($ a : ty , $ b : ty) ;) *) => { $ (impl < Input > CheckOutput < Input > for ($ a , $ b) where Input : Hex + fmt :: Debug , SpecialCase : MaybeOverride < Input >, { fn validate <'a > (self , expected : Self , input : Input , ctx : & CheckCtx ,) -> TestResult { self . 0 . validate (expected . 0 , input , ctx) . and_then (| () | self . 1 . validate (expected . 1 , input , ctx)) . with_context (|| format ! ("full context:\
                            \n    input:    {input:?} {ibits}\
                            \n    as hex:   {ihex}\
                            \n    as bits:  {ibits}\
                            \n    expected: {expected:?} {expbits}\
                            \n    actual:   {self:?} {actbits}\
                            " , ihex = input . hexf () , ibits = input . hex () , expbits = expected . hex () , actbits = self . hex () ,)) } }) * } ; }
};
}
