// Generated macro for lowercase (function)
macro_rules! Depcrate_global_rnglowercase {
() => {
// Module: crate::global_rng
// Provides: {"lowercase"}
// Dependencies: {}
# [doc = " Generates a random `char` in range a-z."] # [inline] pub fn lowercase () -> char { with_rng (| r | r . lowercase ()) }
};
}
