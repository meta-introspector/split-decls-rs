// Generated macro for impl_6 (impl)
macro_rules! Depcrate_errorimpl_6 {
() => {
// Module: crate::error
// Provides: {"impl_6"}
// Dependencies: {}
impl From < u64 > for AddressError { fn from (error : u64) -> Self { match error { 0 => AddressError :: MaxSeedLengthExceeded , 1 => AddressError :: InvalidSeeds , 2 => AddressError :: IllegalOwner , _ => panic ! ("Unsupported AddressError") , } } }
};
}
