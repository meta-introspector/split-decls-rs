// Generated macro for impl_46 (impl)
macro_rules! Depcrate_eventsimpl_46 {
() => {
// Module: crate::events
// Provides: {"impl_46"}
// Dependencies: {}
impl Event { # [doc = " Returns a new `Event` object with the provided time and data."] pub fn with_time (time : f32 , data : EventData) -> Self { Self :: with_time_ex (time , data , Default :: default ()) } # [doc = " Returns a new `Event` object with the provided time, data and ex_data."] pub fn with_time_ex (time : f32 , data : EventData , ex_data : ExData) -> Self { let ty = EventType :: from (& data) ; Event { time , data , ex_data , protocol_type : Default :: default () , group_id : Default :: default () , time_format : Default :: default () , ty , } } }
};
}
