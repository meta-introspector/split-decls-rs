// Generated macro for crates (function)
macro_rules! Depcratecrates {
() => {
// Module: crate
// Provides: {"crates"}
// Dependencies: {}
pub fn crates < P : AsRef < Path > > (path : P) -> Vec < Crate > { let mut crates = find (path) ; crates . sort () ; crates }
};
}
