// Generated macro for check_vec (function)
macro_rules! Depcrate_methods_drain_collectcheck_vec {
() => {
// Module: crate::methods::drain_collect
// Provides: {"check_vec"}
// Dependencies: {}
# [doc = " Checks `std::{vec::Vec, collections::VecDeque}`."] fn check_vec (cx : & LateContext < '_ > , args : & [Expr < '_ >] , expr : Ty < '_ > , recv : Ty < '_ > , recv_path : & Path < '_ >) -> bool { (types_match_diagnostic_item (cx , expr , recv , sym :: Vec) || types_match_diagnostic_item (cx , expr , recv , sym :: VecDeque)) && matches ! (args , [arg] if is_range_full (cx , arg , Some (recv_path))) }
};
}
