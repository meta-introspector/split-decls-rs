// Generated macro for QlogEvent (enum)
macro_rules! Depcrate_recordreplay_qlogQlogEvent {
() => {
// Module: crate::recordreplay::qlog
// Provides: {"QlogEvent"}
// Dependencies: {}
# [doc = " A qlog event representation using either the official RFC format or the"] # [doc = " catch-al JSON event."] pub enum QlogEvent { Event { data : Box < EventData > , ex_data : ExData , } , JsonEvent (JsonEvent) , }
};
}
