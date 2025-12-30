// Generated macro for spin_write (function)
macro_rules! Depcrate_interact_sessionspin_write {
() => {
// Module: crate::interact::session
// Provides: {"spin_write"}
// Dependencies: {}
fn spin_write < W > (mut writer : W , buf : & [u8]) -> std :: io :: Result < () > where W : Write , { loop { match writer . write_all (buf) { Ok (_) => return Ok (()) , Err (err) => { if err . kind () != ErrorKind :: WouldBlock { return Err (err) ; } } } } }
};
}
