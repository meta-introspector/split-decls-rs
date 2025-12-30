// Generated macro for UMASK (static)
macro_rules! Depcrate_fsUMASK {
() => {
// Module: crate::fs
// Provides: {"UMASK"}
// Dependencies: {}
static UMASK : InterruptSpinMutex < AccessPermission > = InterruptSpinMutex :: new (AccessPermission :: from_bits_retain (0o777)) ;
};
}
