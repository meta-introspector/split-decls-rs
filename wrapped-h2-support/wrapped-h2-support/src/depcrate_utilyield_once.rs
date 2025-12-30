// Generated macro for yield_once (function)
macro_rules! Depcrate_utilyield_once {
() => {
// Module: crate::util
// Provides: {"yield_once"}
// Dependencies: {}
pub async fn yield_once () { let mut yielded = false ; futures :: future :: poll_fn (move | cx | { if yielded { Poll :: Ready (()) } else { yielded = true ; cx . waker () . clone () . wake () ; Poll :: Pending } }) . await ; }
};
}
