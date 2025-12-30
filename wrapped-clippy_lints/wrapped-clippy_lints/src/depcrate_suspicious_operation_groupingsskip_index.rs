// Generated macro for skip_index (function)
macro_rules! Depcrate_suspicious_operation_groupingsskip_index {
() => {
// Module: crate::suspicious_operation_groupings
// Provides: {"skip_index"}
// Dependencies: {}
fn skip_index < A , Iter > (iter : Iter , index : usize) -> impl Iterator < Item = A > where Iter : Iterator < Item = A > , { iter . enumerate () . filter_map (move | (i , a) | if i == index { None } else { Some (a) }) }
};
}
