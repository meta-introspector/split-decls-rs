// Generated macro for Inner (struct)
macro_rules! Depcrate_mockInner {
() => {
// Module: crate::mock
// Provides: {"Inner"}
// Dependencies: {}
# [derive (Debug)] struct Inner { # [doc = " Data written by the test case to the h2 lib."] rx : Vec < u8 > , # [doc = " Notify when data is ready to be received."] rx_task : Option < Waker > , # [doc = " Data written by the `h2` library to be read by the test case."] tx : Vec < u8 > , # [doc = " Notify when data is written. This notifies the test case waiters."] tx_task : Option < Waker > , # [doc = " Number of bytes that can be written before `write` returns `Poll::Pending`."] tx_rem : usize , # [doc = " Task to notify when write capacity becomes available."] tx_rem_task : Option < Waker > , # [doc = " True when the pipe is closed."] closed : bool , # [doc = " Trigger an `UnexpectedEof` error on read"] unexpected_eof : bool , }
};
}
