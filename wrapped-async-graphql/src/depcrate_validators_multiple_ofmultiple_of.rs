// Generated macro for multiple_of (function)
macro_rules! Depcrate_validators_multiple_ofmultiple_of {
() => {
// Module: crate::validators::multiple_of
// Provides: {"multiple_of"}
// Dependencies: {}
pub fn multiple_of < T , N > (value : & T , n : N) -> Result < () , InputValueError < T > > where T : AsPrimitive < N > + InputType , N : Rem < Output = N > + Zero + Display + Copy + PartialEq + 'static , { let value = value . as_ () ; if ! value . is_zero () && value % n == N :: zero () { Ok (()) } else { Err (format ! ("the value must be a multiple of {}." , n) . into ()) } }
};
}
