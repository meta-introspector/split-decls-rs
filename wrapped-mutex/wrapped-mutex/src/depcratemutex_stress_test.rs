// Generated macro for mutex_stress_test (function)
macro_rules! Depcratemutex_stress_test {
() => {
// Module: crate
// Provides: {"mutex_stress_test"}
// Dependencies: {}
fn mutex_stress_test (no_threads : usize) { println ! ("Stress mutex with {no_threads} threads!") ; let counter = Arc :: new (Mutex :: new (0)) ; let barrier = Arc :: new (SpinBarrier :: new (no_threads)) ; let handles = (0 .. no_threads) . map (| _ | { let barrier = barrier . clone () ; let counter = counter . clone () ; thread :: spawn (move | | { let now = Instant :: now () ; for _ in 0 .. NUMBER_OF_ITERATIONS { let mut guard = counter . lock () . unwrap () ; * guard += 1 ; } let _ = now . elapsed () ; barrier . wait () ; let now = Instant :: now () ; for _ in 0 .. NUMBER_OF_ITERATIONS { let mut guard = counter . lock () . unwrap () ; * guard += 1 ; } now . elapsed () }) }) . collect :: < Vec < _ > > () ; let durations = handles . into_iter () . map (| handle | handle . join () . unwrap ()) . collect :: < Vec < _ > > () ; assert_eq ! (* counter . lock () . unwrap () , 2 * NUMBER_OF_ITERATIONS * no_threads) ; let average = durations . iter () . sum :: < Duration > () / u32 :: try_from (no_threads) . unwrap () / u32 :: try_from (NUMBER_OF_ITERATIONS * no_threads) . unwrap () ; log_benchmark_data_with_group (& format ! ("{no_threads} Threads") , "ns" , average . as_nanos () as f64 , "Mutex Stress Test Average Time per Iteration" ,) ; }
};
}
