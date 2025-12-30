// Generated macro for Session (struct)
macro_rules! Depcrate_sessionSession {
() => {
// Module: crate::session
// Provides: {"Session"}
// Dependencies: {}
# [doc = " An instance of this object is a session that can be"] # [doc = " used to record changes to a database."] pub struct Session < 'conn > { phantom : PhantomData < & 'conn Connection > , s : * mut ffi :: sqlite3_session , filter : Filter , }
};
}
