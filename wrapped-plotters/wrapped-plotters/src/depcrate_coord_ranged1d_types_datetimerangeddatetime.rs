// Generated macro for RangedDateTime (struct)
macro_rules! Depcrate_coord_ranged1d_types_datetimeRangedDateTime {
() => {
// Module: crate::coord::ranged1d::types::datetime
// Provides: {"RangedDateTime"}
// Dependencies: {}
# [doc = " The ranged coordinate for the date and time"] # [derive (Clone)] pub struct RangedDateTime < DT : Datelike + Timelike + TimeValue > (DT , DT) ;
};
}
