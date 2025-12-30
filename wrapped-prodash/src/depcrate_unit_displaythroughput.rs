// Generated macro for Throughput (struct)
macro_rules! Depcrate_unit_displayThroughput {
() => {
// Module: crate::unit::display
// Provides: {"Throughput"}
// Dependencies: {}
# [doc = " A structure able to display throughput, a value change within a given duration."] # [derive (Clone , Eq , PartialEq , Ord , PartialOrd , Debug)] pub struct Throughput { # [doc = " The change of value between the current value and the previous one."] pub value_change_in_timespan : Step , # [doc = " The amount of time passed between the previous and the current value."] pub timespan : std :: time :: Duration , }
};
}
