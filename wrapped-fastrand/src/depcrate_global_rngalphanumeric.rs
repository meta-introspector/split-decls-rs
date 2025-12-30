// Generated macro for alphanumeric (function)
macro_rules! Depcrate_global_rngalphanumeric {
() => {
// Module: crate::global_rng
// Provides: {"alphanumeric"}
// Dependencies: {}
# [doc = " Generates a random `char` in ranges a-z, A-Z and 0-9."] # [inline] pub fn alphanumeric () -> char { with_rng (| r | r . alphanumeric ()) }
};
}
