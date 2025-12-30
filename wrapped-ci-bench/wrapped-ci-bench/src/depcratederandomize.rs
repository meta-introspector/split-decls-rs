// Generated macro for derandomize (function)
macro_rules! Depcratederandomize {
() => {
// Module: crate
// Provides: {"derandomize"}
// Dependencies: {}
fn derandomize (base : CryptoProvider) -> CryptoProvider { CryptoProvider { secure_random : & NotRandom , .. base } }
};
}
