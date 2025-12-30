// Generated macro for Direction (struct)
macro_rules! Depcrate_reactorDirection {
() => {
// Module: crate::reactor
// Provides: {"Direction"}
// Dependencies: {}
# [doc = " A read or write direction."] # [derive (Debug , Default)] struct Direction { # [doc = " Last reactor tick that delivered an event."] tick : usize , # [doc = " Ticks remembered by `Async::poll_readable()` or `Async::poll_writable()`."] ticks : Option < (usize , usize) > , # [doc = " Waker stored by `Async::poll_readable()` or `Async::poll_writable()`."] waker : Option < Waker > , # [doc = " Wakers of tasks waiting for the next event."] # [doc = ""] # [doc = " Registered by `Async::readable()` and `Async::writable()`."] wakers : Slab < Option < Waker > > , }
};
}
