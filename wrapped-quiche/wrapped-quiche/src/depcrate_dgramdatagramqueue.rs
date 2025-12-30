// Generated macro for DatagramQueue (struct)
macro_rules! Depcrate_dgramDatagramQueue {
() => {
// Module: crate::dgram
// Provides: {"DatagramQueue"}
// Dependencies: {}
# [doc = " Keeps track of DATAGRAM frames."] # [derive (Default)] pub struct DatagramQueue { queue : Option < VecDeque < Vec < u8 > > > , queue_max_len : usize , queue_bytes_size : usize , }
};
}
