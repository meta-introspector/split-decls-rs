// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let n = 100000000 ; let mut times : Vec < Duration > = Vec :: new () ; for i in 1 ..= 8 { let now = Instant :: now () ; black_box (calc_pi_multi_threaded (black_box (n) , black_box (i))) ; let elapsed = now . elapsed () ; times . push (elapsed) ; } let speedup1_2 = times [0] . as_secs_f64 () / times [1] . as_secs_f64 () ; let speedup1_4 = times [0] . as_secs_f64 () / times [2] . as_secs_f64 () ; let speedup1_8 = times [0] . as_secs_f64 () / times [7] . as_secs_f64 () ; let efficiency1_2 = speedup1_2 / 2.0 ; let efficiency1_4 = speedup1_4 / 4.0 ; let efficiency1_8 = speedup1_8 / 8.0 ; log_benchmark_data ("2 Threads" , "%" , efficiency1_2 * 100.0) ; log_benchmark_data ("4 Threads" , "%" , efficiency1_4 * 100.0) ; log_benchmark_data ("8 Threads" , "%" , efficiency1_8 * 100.0) ; }
};
}
