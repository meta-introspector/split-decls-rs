// Generated macro for Source (struct)
macro_rules! Depcrate_reactorSource {
() => {
// Module: crate::reactor
// Provides: {"Source"}
// Dependencies: {}
# [doc = " A registered source of I/O events."] # [derive (Debug)] pub (crate) struct Source { # [doc = " This source's registration into the reactor."] registration : Registration , # [doc = " The key of this source obtained during registration."] key : usize , # [doc = " Inner state with registered wakers."] state : Mutex < [Direction ; 2] > , }
};
}
