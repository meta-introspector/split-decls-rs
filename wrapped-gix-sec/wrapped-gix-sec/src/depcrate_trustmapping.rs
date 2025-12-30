// Generated macro for Mapping (struct)
macro_rules! Depcrate_trustMapping {
() => {
// Module: crate::trust
// Provides: {"Mapping"}
// Dependencies: {}
# [doc = " Associate instructions for how to deal with various `Trust` levels as they are encountered in the wild."] pub struct Mapping < T > { # [doc = " The value for fully trusted resources."] pub full : T , # [doc = " The value for resources with reduced trust."] pub reduced : T , }
};
}
