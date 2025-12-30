// Generated macro for prop_assume (macro)
macro_rules! Depcrate_sugarprop_assume {
() => {
// Module: crate::sugar
// Provides: {"prop_assume"}
// Dependencies: {}
# [doc = " Rejects the test input if assumptions are not met."] # [doc = ""] # [doc = " Used directly within a function defined with `proptest!` or in any function"] # [doc = " returning `Result<_, TestCaseError>`."] # [doc = ""] # [doc = " This is invoked as `prop_assume!(condition, format, args...)`. `condition`"] # [doc = " is evaluated; if it is false, `Err(TestCaseError::Reject)` is returned. The"] # [doc = " message includes the point of invocation and the format message. `format`"] # [doc = " and `args` may be omitted to simply use the condition itself as the"] # [doc = " message."] # [macro_export] macro_rules ! prop_assume { ($ expr : expr) => { $ crate :: prop_assume ! ($ expr , "{}" , :: core :: stringify ! ($ expr)) } ; ($ expr : expr , $ fmt : tt $ (, $ fmt_arg : expr) ,* $ (,) ?) => { if !$ expr { extern crate alloc ; return :: core :: result :: Result :: Err ($ crate :: test_runner :: TestCaseError :: reject (alloc :: format ! (:: core :: concat ! ("{}:{}:{}: " , $ fmt) , :: core :: file ! () , :: core :: line ! () , :: core :: column ! () $ (, $ fmt_arg) *))) ; } } ; }
};
}
