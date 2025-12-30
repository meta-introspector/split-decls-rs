// Generated macro for macro_547 (macro)
macro_rules! Depcrate_server_conn_automacro_547 {
() => {
// Module: crate::server::conn::auto
// Provides: {"macro_547"}
// Dependencies: {}
pin_project ! { struct ReadVersion < I > { io : Option < I >, buf : [MaybeUninit < u8 >; 24] , filled : usize , version : Version , cancelled : bool , # [pin] _pin : PhantomPinned , } }
};
}
