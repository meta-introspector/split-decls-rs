// Generated macro for find_map_first_or_last_or_any (function)
macro_rules! Depcrate_iter_testfind_map_first_or_last_or_any {
() => {
// Module: crate::iter::test
// Provides: {"find_map_first_or_last_or_any"}
// Dependencies: {}
# [test] fn find_map_first_or_last_or_any () { let mut a : Vec < i32 > = vec ! [] ; assert ! (a . par_iter () . find_map_any (half_if_positive) . is_none ()) ; assert ! (a . par_iter () . find_map_first (half_if_positive) . is_none ()) ; assert ! (a . par_iter () . find_map_last (half_if_positive) . is_none ()) ; a = (- 1024 .. - 3) . collect () ; assert ! (a . par_iter () . find_map_any (half_if_positive) . is_none ()) ; assert ! (a . par_iter () . find_map_first (half_if_positive) . is_none ()) ; assert ! (a . par_iter () . find_map_last (half_if_positive) . is_none ()) ; assert ! (a . par_iter () . find_map_any (half_if_negative) . is_some ()) ; assert_eq ! (a . par_iter () . find_map_first (half_if_negative) , Some (- 512_i32)) ; assert_eq ! (a . par_iter () . find_map_last (half_if_negative) , Some (- 2_i32)) ; a . append (& mut (2 .. 1025) . collect ()) ; assert ! (a . par_iter () . find_map_any (half_if_positive) . is_some ()) ; assert_eq ! (a . par_iter () . find_map_first (half_if_positive) , Some (1_i32)) ; assert_eq ! (a . par_iter () . find_map_last (half_if_positive) , Some (512_i32)) ; fn half_if_positive (x : & i32) -> Option < i32 > { if * x > 0 { Some (x / 2) } else { None } } fn half_if_negative (x : & i32) -> Option < i32 > { if * x < 0 { Some (x / 2) } else { None } } }
};
}
