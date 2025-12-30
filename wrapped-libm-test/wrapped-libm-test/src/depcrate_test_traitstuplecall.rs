// Generated macro for TupleCall (trait)
macro_rules! Depcrate_test_traitsTupleCall {
() => {
// Module: crate::test_traits
// Provides: {"TupleCall"}
// Dependencies: {}
# [doc = " Trait for calling a function with a tuple as arguments."] # [doc = ""] # [doc = " Implemented on the tuple with the function signature as the generic (so we can use the same"] # [doc = " tuple for multiple signatures)."] pub trait TupleCall < Func > : fmt :: Debug { type Output ; fn call (self , f : Func) -> Self :: Output ; # [doc = " Intercept panics and print the input to stderr before continuing."] fn call_intercept_panics (self , f : Func) -> Self :: Output where Self : RefUnwindSafe + Copy , Func : UnwindSafe , { let res = panic :: catch_unwind (| | self . call (f)) ; match res { Ok (v) => v , Err (e) => { eprintln ! ("panic with the following input: {self:?}") ; panic :: resume_unwind (e) } } } }
};
}
