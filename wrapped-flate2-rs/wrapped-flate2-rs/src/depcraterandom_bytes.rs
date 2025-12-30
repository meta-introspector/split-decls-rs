// Generated macro for random_bytes (function)
macro_rules! Depcraterandom_bytes {
() => {
// Module: crate
// Provides: {"random_bytes"}
// Dependencies: {}
# [cfg (test)] fn random_bytes () -> impl Iterator < Item = u8 > { use rand :: Rng ; use std :: iter ; iter :: repeat (()) . map (| _ | rand :: rng () . random ()) }
};
}
