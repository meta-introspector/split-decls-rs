// Generated macro for RestoreOnDrop (struct)
macro_rules! Depcrate_global_rngRestoreOnDrop {
() => {
// Module: crate::global_rng
// Provides: {"RestoreOnDrop"}
// Dependencies: {}
# [doc = " Make sure the original RNG is restored even on panic."] struct RestoreOnDrop < 'a > { rng : & 'a Cell < Rng > , current : Rng , }
};
}
