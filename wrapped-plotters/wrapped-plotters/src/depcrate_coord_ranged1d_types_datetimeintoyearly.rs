// Generated macro for IntoYearly (trait)
macro_rules! Depcrate_coord_ranged1d_types_datetimeIntoYearly {
() => {
// Module: crate::coord::ranged1d::types::datetime
// Provides: {"IntoYearly"}
// Dependencies: {}
# [doc = " The trait that converts a normal date coord into a yearly one"] pub trait IntoYearly < T : TimeValue > { # [doc = " Converts a normal date coord into a yearly one"] fn yearly (self) -> Yearly < T > ; }
};
}
