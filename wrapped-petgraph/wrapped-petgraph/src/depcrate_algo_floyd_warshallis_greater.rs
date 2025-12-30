// Generated macro for is_greater (function)
macro_rules! Depcrate_algo_floyd_warshallis_greater {
() => {
// Module: crate::algo::floyd_warshall
// Provides: {"is_greater"}
// Dependencies: {}
# [doc = " Helper to check if the distance map is greater then a specific value"] fn is_greater < K : PartialOrd > (m_dist : & mut Option < Vec < Vec < K > > > , i : usize , j : usize , value : K ,) -> bool { if let Some (dist) = m_dist { return dist [i] [j] > value ; } false }
};
}
