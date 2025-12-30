// Generated macro for transpose_result (function)
macro_rules! Depcrate_adaptorstranspose_result {
() => {
// Module: crate::adaptors
// Provides: {"transpose_result"}
// Dependencies: {}
fn transpose_result < T , E > (result : Result < Option < T > , E >) -> Option < Result < T , E > > { match result { Ok (Some (v)) => Some (Ok (v)) , Ok (None) => None , Err (e) => Some (Err (e)) , } }
};
}
