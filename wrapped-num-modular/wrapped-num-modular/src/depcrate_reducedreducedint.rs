// Generated macro for ReducedInt (struct)
macro_rules! Depcrate_reducedReducedInt {
() => {
// Module: crate::reduced
// Provides: {"ReducedInt"}
// Dependencies: {}
# [doc = " An integer in a modulo ring"] # [derive (Debug , Clone , Copy)] pub struct ReducedInt < T , R : Reducer < T > > { # [doc = " The reduced representation of the integer in a modulo ring."] a : T , # [doc = " The reducer for the integer"] r : R , }
};
}
