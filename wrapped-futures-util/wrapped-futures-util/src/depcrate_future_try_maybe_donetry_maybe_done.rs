// Generated macro for try_maybe_done (function)
macro_rules! Depcrate_future_try_maybe_donetry_maybe_done {
() => {
// Module: crate::future::try_maybe_done
// Provides: {"try_maybe_done"}
// Dependencies: {}
# [doc = " Wraps a future into a `TryMaybeDone`"] pub fn try_maybe_done < Fut : TryFuture > (future : Fut) -> TryMaybeDone < Fut > { assert_future :: < Result < () , Fut :: Error > , _ > (TryMaybeDone :: Future (future)) }
};
}
