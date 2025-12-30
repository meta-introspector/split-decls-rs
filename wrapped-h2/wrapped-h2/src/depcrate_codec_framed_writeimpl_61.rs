// Generated macro for impl_61 (impl)
macro_rules! Depcrate_codec_framed_writeimpl_61 {
() => {
// Module: crate::codec::framed_write
// Provides: {"impl_61"}
// Dependencies: {}
impl < T , B > FramedWrite < T , B > { # [doc = " Returns the max frame size that can be sent"] pub fn max_frame_size (& self) -> usize { self . encoder . max_frame_size () } # [doc = " Set the peer's max frame size."] pub fn set_max_frame_size (& mut self , val : usize) { assert ! (val <= frame :: MAX_MAX_FRAME_SIZE as usize) ; self . encoder . max_frame_size = val as FrameSize ; } # [doc = " Set the peer's header table size."] pub fn set_header_table_size (& mut self , val : usize) { self . encoder . hpack . update_max_size (val) ; } # [doc = " Retrieve the last data frame that has been sent"] pub fn take_last_data_frame (& mut self) -> Option < frame :: Data < B > > { self . encoder . last_data_frame . take () } pub fn get_mut (& mut self) -> & mut T { & mut self . inner } }
};
}
