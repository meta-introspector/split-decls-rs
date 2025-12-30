// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
# [doc = " Default impl for `FpsWidget`"] # [doc = ""] # [doc = " Manual impl is required because we need to initialize the `last_instant` field to the current"] # [doc = " instant."] impl Default for FpsWidget { fn default () -> Self { Self { frame_count : 0 , last_instant : Instant :: now () , fps : None , } } }
};
}
