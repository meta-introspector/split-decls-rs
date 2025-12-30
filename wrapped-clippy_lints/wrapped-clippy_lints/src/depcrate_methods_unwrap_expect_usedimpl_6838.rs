// Generated macro for impl_6838 (impl)
macro_rules! Depcrate_methods_unwrap_expect_usedimpl_6838 {
() => {
// Module: crate::methods::unwrap_expect_used
// Provides: {"impl_6838"}
// Dependencies: {}
impl Variant { fn method_name (self , is_err : bool) -> & 'static str { match (self , is_err) { (Variant :: Unwrap , true) => "unwrap_err" , (Variant :: Unwrap , false) => "unwrap" , (Variant :: Expect , true) => "expect_err" , (Variant :: Expect , false) => "expect" , } } fn lint (self) -> & 'static Lint { match self { Variant :: Unwrap => UNWRAP_USED , Variant :: Expect => EXPECT_USED , } } }
};
}
