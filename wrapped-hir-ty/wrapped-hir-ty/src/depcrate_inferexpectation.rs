// Generated macro for Expectation (enum)
macro_rules! Depcrate_inferExpectation {
() => {
// Module: crate::infer
// Provides: {"Expectation"}
// Dependencies: {}
# [doc = " When inferring an expression, we propagate downward whatever type hint we"] # [doc = " are able in the form of an `Expectation`."] # [derive (Clone , PartialEq , Eq , Debug)] pub (crate) enum Expectation < 'db > { None , HasType (Ty < 'db >) , Castable (Ty < 'db >) , RValueLikeUnsized (Ty < 'db >) , }
};
}
