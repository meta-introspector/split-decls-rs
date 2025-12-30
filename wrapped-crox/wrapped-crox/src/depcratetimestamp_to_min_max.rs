// Generated macro for timestamp_to_min_max (function)
macro_rules! Depcratetimestamp_to_min_max {
() => {
// Module: crate
// Provides: {"timestamp_to_min_max"}
// Dependencies: {}
fn timestamp_to_min_max (timestamp : Timestamp) -> (SystemTime , SystemTime) { match timestamp { Timestamp :: Instant (t) => (t , t) , Timestamp :: Interval { start , end } => { (cmp :: min (start , end) , cmp :: max (start , end)) } } }
};
}
