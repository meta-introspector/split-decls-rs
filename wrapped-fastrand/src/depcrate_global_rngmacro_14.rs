// Generated macro for macro_14 (macro)
macro_rules! Depcrate_global_rngmacro_14 {
() => {
// Module: crate::global_rng
// Provides: {"macro_14"}
// Dependencies: {}
std :: thread_local ! { static RNG : Cell < Rng > = Cell :: new (Rng (random_seed () . unwrap_or (DEFAULT_RNG_SEED))) ; }
};
}
