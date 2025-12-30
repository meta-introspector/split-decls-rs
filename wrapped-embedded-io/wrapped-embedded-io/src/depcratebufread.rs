// Generated macro for BufRead (trait)
macro_rules! DepcrateBufRead {
() => {
// Module: crate
// Provides: {"BufRead"}
// Dependencies: {}
# [doc = " Blocking buffered reader."] # [doc = ""] # [doc = " This trait is the `embedded-io` equivalent of [`std::io::BufRead`]."] pub trait BufRead : Read { # [doc = " Return the contents of the internal buffer, filling it with more data from the inner reader if it is empty."] # [doc = ""] # [doc = " If no bytes are currently available to read, this function blocks until at least one byte is available."] # [doc = ""] # [doc = " If the reader is at end-of-file (EOF), an empty slice is returned. There is no guarantee that a reader at EOF"] # [doc = " will always be so in the future, for example a reader can stop being at EOF if another process appends"] # [doc = " more bytes to the underlying file."] fn fill_buf (& mut self) -> Result < & [u8] , Self :: Error > ; # [doc = " Tell this buffer that `amt` bytes have been consumed from the buffer, so they should no longer be returned in calls to `fill_buf`."] fn consume (& mut self , amt : usize) ; }
};
}
