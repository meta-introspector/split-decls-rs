// Generated macro for RunningDtorState (struct)
macro_rules! Depcrate_shims_tlsRunningDtorState {
() => {
// Module: crate::shims::tls
// Provides: {"RunningDtorState"}
// Dependencies: {}
# [derive (Default , Debug)] struct RunningDtorState { # [doc = " The last TlsKey used to retrieve a TLS destructor. `None` means that we"] # [doc = " have not tried to retrieve a TLS destructor yet or that we already tried"] # [doc = " all keys."] last_key : Option < TlsKey > , }
};
}
