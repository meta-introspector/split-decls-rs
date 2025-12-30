// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl < 'a , S > Connection < 'a , S > where S : ScalarValue + Send + Sync + 'a , { # [doc = " Creates new [`Connection`] from values stream and errors"] pub fn from_stream (stream : Value < ValuesStream < 'a , S > > , errors : Vec < ExecutionError < S > >) -> Self { Self { stream : whole_responses_stream (stream , errors) , } } }
};
}
