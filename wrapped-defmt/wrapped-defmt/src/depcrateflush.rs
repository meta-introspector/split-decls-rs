// Generated macro for flush (function)
macro_rules! Depcrateflush {
() => {
// Module: crate
// Provides: {"flush"}
// Dependencies: {}
# [doc = " Block until host has read all pending data."] # [doc = ""] # [doc = " The flush operation will not fail, but might not succeed in flushing _all_ pending data. It is"] # [doc = " implemented as a \"best effort\" operation."] # [doc = ""] # [doc = " This calls the method `flush` of the used \"global [`Logger`]\". The logger is likely provided by"] # [doc = " [`defmt-rtt`](https://crates.io/crates/defmt-rtt) or [`defmt-itm`](https://crates.io/crates/defmt-itm)."] pub fn flush () { match () { # [cfg (feature = "unstable-test")] () => { } # [cfg (not (feature = "unstable-test"))] () => { extern "Rust" { fn _defmt_acquire () ; fn _defmt_flush () ; fn _defmt_release () ; } unsafe { _defmt_acquire () ; _defmt_flush () ; _defmt_release () } } } }
};
}
