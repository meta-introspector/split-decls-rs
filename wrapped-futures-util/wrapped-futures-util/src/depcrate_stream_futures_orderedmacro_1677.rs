// Generated macro for macro_1677 (macro)
macro_rules! Depcrate_stream_futures_orderedmacro_1677 {
() => {
// Module: crate::stream::futures_ordered
// Provides: {"macro_1677"}
// Dependencies: {}
pin_project ! { # [must_use = "futures do nothing unless you `.await` or poll them"] # [derive (Debug)] struct OrderWrapper < T > { # [pin] data : T , index : i64 , } }
};
}
