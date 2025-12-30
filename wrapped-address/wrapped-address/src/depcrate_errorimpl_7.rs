// Generated macro for impl_7 (impl)
macro_rules! Depcrate_errorimpl_7 {
() => {
// Module: crate::error
// Provides: {"impl_7"}
// Dependencies: {}
impl From < AddressError > for ProgramError { fn from (error : AddressError) -> Self { match error { AddressError :: MaxSeedLengthExceeded => Self :: MaxSeedLengthExceeded , AddressError :: InvalidSeeds => Self :: InvalidSeeds , AddressError :: IllegalOwner => Self :: IllegalOwner , } } }
};
}
