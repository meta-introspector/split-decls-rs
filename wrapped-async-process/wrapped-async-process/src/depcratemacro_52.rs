// Generated macro for macro_52 (macro)
macro_rules! Depcratemacro_52 {
() => {
// Module: crate
// Provides: {"macro_52"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (windows)] { fn wrap < T > (io : T) -> io :: Result < Unblock < T >> { Ok (Unblock :: new (io)) } } else if # [cfg (unix)] { # [doc = " Wrap a file descriptor into a non-blocking I/O type."] fn wrap < T : std :: os :: unix :: io :: AsFd > (io : T) -> io :: Result < Async < T >> { Async :: new (io) } } }
};
}
