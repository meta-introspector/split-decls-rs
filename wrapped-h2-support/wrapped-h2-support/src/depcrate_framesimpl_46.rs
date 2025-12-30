// Generated macro for impl_46 (impl)
macro_rules! Depcrate_framesimpl_46 {
() => {
// Module: crate::frames
// Provides: {"impl_46"}
// Dependencies: {}
impl Mock < frame :: Settings > { pub fn max_concurrent_streams (mut self , max : u32) -> Self { self . 0 . set_max_concurrent_streams (Some (max)) ; self } pub fn max_frame_size (mut self , val : u32) -> Self { self . 0 . set_max_frame_size (Some (val)) ; self } pub fn initial_window_size (mut self , val : u32) -> Self { self . 0 . set_initial_window_size (Some (val)) ; self } pub fn max_header_list_size (mut self , val : u32) -> Self { self . 0 . set_max_header_list_size (Some (val)) ; self } pub fn disable_push (mut self) -> Self { self . 0 . set_enable_push (false) ; self } pub fn enable_connect_protocol (mut self , val : u32) -> Self { self . 0 . set_enable_connect_protocol (Some (val)) ; self } pub fn header_table_size (mut self , val : u32) -> Self { self . 0 . set_header_table_size (Some (val)) ; self } }
};
}
