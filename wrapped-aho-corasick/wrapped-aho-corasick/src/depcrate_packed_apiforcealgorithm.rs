// Generated macro for ForceAlgorithm (enum)
macro_rules! Depcrate_packed_apiForceAlgorithm {
() => {
// Module: crate::packed::api
// Provides: {"ForceAlgorithm"}
// Dependencies: {}
# [doc = " An internal option for forcing the use of a particular packed algorithm."] # [doc = ""] # [doc = " When an algorithm is forced, if a searcher could not be constructed for it,"] # [doc = " then no searcher will be returned even if an alternative algorithm would"] # [doc = " work."] # [derive (Clone , Debug)] enum ForceAlgorithm { Teddy , RabinKarp , }
};
}
