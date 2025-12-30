// Generated macro for Monthly (struct)
macro_rules! Depcrate_coord_ranged1d_types_datetimeMonthly {
() => {
// Module: crate::coord::ranged1d::types::datetime
// Provides: {"Monthly"}
// Dependencies: {}
# [doc = " Indicates the coord has a monthly resolution"] # [doc = ""] # [doc = " Note: since month doesn't have a constant duration."] # [doc = " We can't use a simple granularity to describe it. Thus we have"] # [doc = " this axis decorator to make it yield monthly key-points."] # [derive (Clone)] pub struct Monthly < T : TimeValue > (Range < T >) ;
};
}
