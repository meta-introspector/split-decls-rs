// Generated macro for check_lengths (function)
macro_rules! Depcrate_iter_testcheck_lengths {
() => {
// Module: crate::iter::test
// Provides: {"check_lengths"}
// Dependencies: {}
# [test] fn check_lengths () { fn check (min : usize , max : usize) { let range = 0 .. 1024 * 1024 ; let min_check = Ord :: min (Ord :: max (min , 1) , range . len ()) ; let max_check = Ord :: max (max , min_check . saturating_add (min_check - 1)) ; assert ! (range . into_par_iter () . with_min_len (min) . with_max_len (max) . fold (|| 0 , | count , _ | count + 1) . all (| c | c >= min_check && c <= max_check) , "check_lengths failed {:?} -> {:?} " , (min , max) , (min_check , max_check)) ; } let lengths = [0 , 1 , 10 , 100 , 1_000 , 10_000 , 100_000 , 1_000_000 , usize :: MAX] ; for & min in & lengths { for & max in & lengths { check (min , max) ; } } }
};
}
