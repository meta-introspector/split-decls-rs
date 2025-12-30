// Generated macro for read_version (function)
macro_rules! Depcrate_server_conn_autoread_version {
() => {
// Module: crate::server::conn::auto
// Provides: {"read_version"}
// Dependencies: {}
fn read_version < I > (io : I) -> ReadVersion < I > where I : Read + Unpin , { ReadVersion { io : Some (io) , buf : [MaybeUninit :: uninit () ; 24] , filled : 0 , version : Version :: H2 , cancelled : false , _pin : PhantomPinned , } }
};
}
