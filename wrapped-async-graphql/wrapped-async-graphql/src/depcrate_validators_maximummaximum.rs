// Generated macro for maximum (function)
macro_rules! Depcrate_validators_maximummaximum {
() => {
// Module: crate::validators::maximum
// Provides: {"maximum"}
// Dependencies: {}
pub fn maximum < T , N > (value : & T , n : N) -> Result < () , InputValueError < T > > where T : AsPrimitive < N > + InputType , N : PartialOrd + Display + Copy + 'static , { if value . as_ () <= n { Ok (()) } else { Err (format ! ("the value is {}, must be less than or equal to {}" , value . as_ () , n) . into ()) } }
};
}
