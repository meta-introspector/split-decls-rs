// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> windows :: core :: Result < () > { unsafe { let uri = CreateUri (w ! ("http://kennykerr.ca") , URI_CREATE_FLAGS :: default () , None) ? ; let domain = uri . GetDomain () ? ; let port = uri . GetPort () ? ; println ! ("{:?} ({port})" , domain) ; Ok (()) } }
};
}
