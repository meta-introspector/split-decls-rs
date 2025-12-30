// Generated macro for check_collections (function)
macro_rules! Depcrate_methods_drain_collectcheck_collections {
() => {
// Module: crate::methods::drain_collect
// Provides: {"check_collections"}
// Dependencies: {}
# [doc = " Checks `std::collections::{HashSet, HashMap, BinaryHeap}`."] fn check_collections (cx : & LateContext < '_ > , expr : Ty < '_ > , recv : Ty < '_ >) -> Option < & 'static str > { types_match_diagnostic_item (cx , expr , recv , sym :: HashSet) . then_some ("HashSet") . or_else (| | types_match_diagnostic_item (cx , expr , recv , sym :: HashMap) . then_some ("HashMap")) . or_else (| | types_match_diagnostic_item (cx , expr , recv , sym :: BinaryHeap) . then_some ("BinaryHeap")) }
};
}
