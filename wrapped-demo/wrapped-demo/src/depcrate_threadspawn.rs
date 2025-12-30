// Generated macro for spawn (function)
macro_rules! Depcrate_threadspawn {
() => {
// Module: crate::thread
// Provides: {"spawn"}
// Dependencies: {}
pub fn spawn () -> io :: Result < () > { eprintln ! () ; let available_parallelism = thread :: available_parallelism () ? ; eprintln ! ("available_parallelism = {available_parallelism}") ; eprint ! ("Thread:") ; let thread_number = available_parallelism . get () * 2 ; let handlers = (0 .. thread_number) . map (| i | { thread :: spawn (move | | { # [derive (Default)] # [repr (align (0x10))] struct Aligned (u8) ; thread_local ! { static THREAD_LOCAL : Cell < Aligned > = const { Cell :: new (Aligned (0x42)) } ; } eprint ! (" {i}") ; thread :: sleep (Duration :: from_millis (u64 :: try_from (thread_number - i) . unwrap () * 10 ,)) ; THREAD_LOCAL . with (| thread_local | { assert_eq ! (0x42 , thread_local . take () . 0) ; }) ; thread :: sleep (Duration :: from_millis (u64 :: try_from (i) . unwrap () * 20)) ; THREAD_LOCAL . with (| thread_local | { assert_eq ! (0 , thread_local . take () . 0) ; }) ; }) }) . collect :: < Vec < _ > > () ; for handler in handlers { handler . join () . unwrap () ; } eprintln ! () ; Ok (()) }
};
}
