// Generated macro for ROUND (macro)
macro_rules! Depcrate_hazardous_stream_chacha20ROUND {
() => {
// Module: crate::hazardous::stream::chacha20
// Provides: {"ROUND"}
// Dependencies: {}
macro_rules ! ROUND { ($ r0 : expr , $ r1 : expr , $ r2 : expr , $ r3 : expr) => { $ r0 = $ r0 . wrapping_add ($ r1) ; $ r3 = ($ r3 ^ $ r0) . rotate_left (16) ; $ r2 = $ r2 . wrapping_add ($ r3) ; $ r1 = ($ r1 ^ $ r2) . rotate_left (12) ; $ r0 = $ r0 . wrapping_add ($ r1) ; $ r3 = ($ r3 ^ $ r0) . rotate_left (8) ; $ r2 = $ r2 . wrapping_add ($ r3) ; $ r1 = ($ r1 ^ $ r2) . rotate_left (7) ; } ; }
};
}
