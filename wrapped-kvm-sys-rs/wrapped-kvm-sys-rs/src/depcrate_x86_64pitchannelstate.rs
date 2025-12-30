// Generated macro for PitChannelState (struct)
macro_rules! Depcrate_x86_64PitChannelState {
() => {
// Module: crate::x86_64
// Provides: {"PitChannelState"}
// Dependencies: {}
# [repr (C)] # [derive (Copy)] pub struct PitChannelState { pub count : u32 , pub latched_count : u16 , pub count_latched : u8 , pub status_latched : u8 , pub status : u8 , pub read_state : u8 , pub write_state : u8 , pub write_latch : u8 , pub rw_mode : u8 , pub mode : u8 , pub bcd : u8 , pub gate : u8 , pub count_load_time : i64 , }
};
}
