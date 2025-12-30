// Generated macro for impl_245 (impl)
macro_rules! Depcrate_executorimpl_245 {
() => {
// Module: crate::executor
// Provides: {"impl_245"}
// Dependencies: {}
impl < S > ExecutionError < S > { # [doc = " Construct a new execution error occuring at the beginning of the query"] pub fn at_origin (error : FieldError < S >) -> ExecutionError < S > { ExecutionError { location : SourcePosition :: new_origin () , path : Vec :: new () , error , } } }
};
}
