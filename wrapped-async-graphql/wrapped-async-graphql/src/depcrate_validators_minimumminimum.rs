// Generated macro for minimum (function)
macro_rules! Depcrate_validators_minimumminimum {
() => {
// Module: crate::validators::minimum
// Provides: {"minimum"}
// Dependencies: {}
pub fn minimum < T , N > (value : & T , n : N) -> Result < () , InputValueError < T > > where T : AsPrimitive < N > + InputType , N : PartialOrd + Display + Copy + 'static , { if value . as_ () >= n { Ok (()) } else { Err (format ! ("the value is {}, must be greater than or equal to {}" , value . as_ () , n) . into ()) } }
};
}
