// Generated macro for ReactorLock (struct)
macro_rules! Depcrate_reactorReactorLock {
() => {
// Module: crate::reactor
// Provides: {"ReactorLock"}
// Dependencies: {}
# [doc = " A lock on the reactor."] pub (crate) struct ReactorLock < 'a > { reactor : & 'a Reactor , events : MutexGuard < 'a , Events > , }
};
}
