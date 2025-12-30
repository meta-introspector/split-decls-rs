// Generated macro for available_buffer_size (function)
macro_rules! Depcrate_channelavailable_buffer_size {
() => {
// Module: crate::channel
// Provides: {"available_buffer_size"}
// Dependencies: {}
# [doc = " How much space is left in the buffer?"] fn available_buffer_size (read_cursor : usize , write_cursor : usize) -> usize { if read_cursor > write_cursor { read_cursor - write_cursor - 1 } else { BUF_SIZE - write_cursor - 1 + read_cursor } }
};
}
