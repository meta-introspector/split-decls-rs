// Generated macro for event_index_to_addr (function)
macro_rules! Depcrateevent_index_to_addr {
() => {
// Module: crate
// Provides: {"event_index_to_addr"}
// Dependencies: {}
fn event_index_to_addr (event_index : usize) -> usize { FILE_HEADER_SIZE + event_index * mem :: size_of :: < RawEvent > () }
};
}
