// Generated macro for impl_125 (impl)
macro_rules! Depcrateimpl_125 {
() => {
// Module: crate
// Provides: {"impl_125"}
// Dependencies: {}
impl < T : FloatCore > NotNan < T > { # [doc = " Create a `NotNan` value."] # [doc = ""] # [doc = " Returns `Err` if `val` is NaN"] pub fn new (val : T) -> Result < Self , FloatIsNan > { match val { ref val if val . is_nan () => Err (FloatIsNan) , val => Ok (NotNan (val)) , } } }
};
}
