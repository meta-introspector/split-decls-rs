// Generated macro for test (module)
macro_rules! Depcrate_coord_ranged1d_combinators_nestedtest {
() => {
// Module: crate::coord::ranged1d::combinators::nested
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_nested_coord () { let coord = (0 .. 10) . nested_coord (| x | 0 .. (x + 1)) ; let range = coord . range () ; assert_eq ! (NestedValue :: Value (0 , 0) .. NestedValue :: Value (10 , 11) , range) ; assert_eq ! (coord . map (& NestedValue :: Category (0) , (0 , 1100)) , 50) ; assert_eq ! (coord . map (& NestedValue :: Value (0 , 0) , (0 , 1100)) , 0) ; assert_eq ! (coord . map (& NestedValue :: Value (5 , 4) , (0 , 1100)) , 567) ; assert_eq ! (coord . size () , (2 + 12) * 11 / 2) ; assert_eq ! (coord . index_of (& NestedValue :: Value (5 , 4)) , Some (24)) ; assert_eq ! (coord . from_index (24) , Some (NestedValue :: Value (5 , 4))) ; } }
};
}
