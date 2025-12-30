// Generated macro for State (struct)
macro_rules! Depcrate_offset_local_tz_info_parserState {
() => {
// Module: crate::offset::local::tz_info::parser
// Provides: {"State"}
// Dependencies: {}
# [doc = " TZif data blocks"] struct State < 'a > { header : Header , # [doc = " Time size in bytes"] time_size : usize , # [doc = " Transition times data block"] transition_times : & 'a [u8] , # [doc = " Transition types data block"] transition_types : & 'a [u8] , # [doc = " Local time types data block"] local_time_types : & 'a [u8] , # [doc = " Time zone names data block"] names : & 'a [u8] , # [doc = " Leap seconds data block"] leap_seconds : & 'a [u8] , # [doc = " UT/local indicators data block"] std_walls : & 'a [u8] , # [doc = " Standard/wall indicators data block"] ut_locals : & 'a [u8] , }
};
}
