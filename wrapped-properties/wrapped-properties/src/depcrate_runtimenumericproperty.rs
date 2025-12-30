// Generated macro for NumericProperty (enum)
macro_rules! Depcrate_runtimeNumericProperty {
() => {
// Module: crate::runtime
// Provides: {"NumericProperty"}
// Dependencies: {}
# [doc = " This type can represent any numeric Unicode property."] # [doc = ""] # [doc = " This is intended to be used in situations where the exact unicode property needed is"] # [doc = " only known at runtime, for example in regex engines."] # [doc = ""] # [doc = " The values are intended to be identical to ICU4C's UProperty enum"] # [non_exhaustive] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] # [allow (dead_code)] # [allow (missing_docs)] enum NumericProperty { NumericValue = 0x3000 , }
};
}
