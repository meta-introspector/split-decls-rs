// Generated macro for transpose (function)
macro_rules! Depcrate_testtranspose {
() => {
// Module: crate::test
// Provides: {"transpose"}
// Dependencies: {}
fn transpose < T , E > (r : Result < Option < T > , E >) -> Option < Result < T , E > > { match r { Ok (Some (x)) => Some (Ok (x)) , Ok (None) => None , Err (e) => Some (Err (e)) , } }
};
}
