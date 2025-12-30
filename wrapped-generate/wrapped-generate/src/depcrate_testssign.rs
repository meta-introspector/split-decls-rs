// Generated macro for sign (function)
macro_rules! Depcrate_testssign {
() => {
// Module: crate::tests
// Provides: {"sign"}
// Dependencies: {}
fn sign (i : i64) -> char { use std :: cmp :: Ordering :: * ; match i . cmp (& 0) { Greater => 'P' , Less => 'N' , Equal => '_' , } }
};
}
