// Generated macro for maybe_yield (function)
macro_rules! Depcratemaybe_yield {
() => {
// Module: crate
// Provides: {"maybe_yield"}
// Dependencies: {}
# [doc = " Yield with some small probability."] fn maybe_yield (rng : & mut fastrand :: Rng , cx : & mut Context < '_ >) -> Poll < () > { if rng . usize (.. 100) == 0 { cx . waker () . wake_by_ref () ; Poll :: Pending } else { Poll :: Ready (()) } }
};
}
