// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> anyhow :: Result < () > { let entries = 8 ; test :: < squeue :: Entry , cqueue :: Entry > (IoUring :: new (entries) ?) ? ; test_sqpoll :: < squeue :: Entry , cqueue :: Entry > (IoUring :: builder () . setup_sqpoll (1000) . build (entries) ? ,) ? ; # [cfg (not (feature = "ci"))] { match IoUring :: < squeue :: Entry128 , cqueue :: Entry > :: builder () . build (entries) { Ok (r) => test (r) ? , Err (e) => { println ! ("IoUring::<squeue::Entry128, cqueue::Entry>::generic_new(entries) failed: {}" , e) ; println ! ("Assume kernel doesn't support the new entry sizes so remaining tests being skipped.") ; return Ok (()) ; } } ; test (IoUring :: < squeue :: Entry , cqueue :: Entry32 > :: builder () . build (entries) ?) ? ; test (IoUring :: < squeue :: Entry128 , cqueue :: Entry32 > :: builder () . build (entries) ?) ? ; } Ok (()) }
};
}
