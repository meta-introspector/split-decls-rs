// Generated macro for Handshaking (enum)
macro_rules! Depcrate_serverHandshaking {
() => {
// Module: crate::server
// Provides: {"Handshaking"}
// Dependencies: {}
# [doc = " Stages of an in-progress handshake."] enum Handshaking < T , B : Buf > { # [doc = " State 1. Connection is flushing pending SETTINGS frame."] Flushing (Instrumented < Flush < T , Prioritized < B > > >) , # [doc = " State 2. Connection is waiting for the client preface."] ReadingPreface (Instrumented < ReadPreface < T , Prioritized < B > > >) , # [doc = " State 3. Handshake is done, polling again would panic."] Done , }
};
}
