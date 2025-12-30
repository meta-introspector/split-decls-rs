// Generated macro for Repr (enum)
macro_rules! Depcrate_registryRepr {
() => {
// Module: crate::registry
// Provides: {"Repr"}
// Dependencies: {}
# [doc = " Note: must not encode `HKEY_PERFORMANCE_DATA` or one of its subkeys."] enum Repr { # [doc = " `HKEY_LOCAL_MACHINE`."] LocalMachine , # [doc = " A subkey of `HKEY_LOCAL_MACHINE`."] Owned (OwnedKey) , }
};
}
