// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> windows :: core :: Result < () > { unsafe { let event = Owned :: new (CreateEventW (None , true , false , None) ?) ; SetEvent (* event) ? ; WaitForSingleObject (* event , 0) ; } Ok (()) }
};
}
