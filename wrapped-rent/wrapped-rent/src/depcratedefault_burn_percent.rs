// Generated macro for DEFAULT_BURN_PERCENT (const)
macro_rules! DepcrateDEFAULT_BURN_PERCENT {
() => {
// Module: crate
// Provides: {"DEFAULT_BURN_PERCENT"}
// Dependencies: {}
# [doc = " Default percentage of collected rent that is burned."] # [doc = ""] # [doc = " Valid values are in the range [0, 100]. The remaining percentage is"] # [doc = " distributed to validators."] # [deprecated (since = "3.1.0" , note = "The concept of rent no longer exists, only rent-exemption")] pub const DEFAULT_BURN_PERCENT : u8 = 50 ;
};
}
