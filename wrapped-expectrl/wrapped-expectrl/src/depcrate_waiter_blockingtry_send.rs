// Generated macro for try_send (function)
macro_rules! Depcrate_waiter_blockingtry_send {
() => {
// Module: crate::waiter::blocking
// Provides: {"try_send"}
// Dependencies: {}
fn try_send (id : usize , msg : Result < Option < u8 > > , sendr : & Sender < (usize , Result < Option < u8 > >) > , buf : & mut Vec < u8 > ,) { match sendr . send ((id , msg)) { Ok (_) => () , Err (err) => { if let Ok (Some (b)) = err . 0 . 1 { buf . push (b) ; } } } }
};
}
