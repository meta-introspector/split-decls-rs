// Generated macro for spin_flush (function)
macro_rules! Depcrate_interact_sessionspin_flush {
() => {
// Module: crate::interact::session
// Provides: {"spin_flush"}
// Dependencies: {}
fn spin_flush < W > (mut writer : W) -> std :: io :: Result < () > where W : Write , { loop { match writer . flush () { Ok (_) => return Ok (()) , Err (err) => { if err . kind () != ErrorKind :: WouldBlock { return Err (err) ; } } } } }
};
}
