// Generated macro for ValuesStream (type)
macro_rules! Depcrate_executorValuesStream {
() => {
// Module: crate::executor
// Provides: {"ValuesStream"}
// Dependencies: {}
# [doc = " Boxed `Stream` yielding `Result<Value<S>, ExecutionError<S>>`"] pub type ValuesStream < 'a , S = DefaultScalarValue > = std :: pin :: Pin < Box < dyn Stream < Item = Result < Value < S > , ExecutionError < S > > > + Send + 'a > > ;
};
}
