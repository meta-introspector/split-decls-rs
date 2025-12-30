// Generated macro for tests (module)
macro_rules! Depcrate_size_hinttests {
() => {
// Module: crate::size_hint
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] fn and () { assert_eq ! ((5 , Some (5)) , super :: and ((2 , Some (2)) , (3 , Some (3)))) ; assert_eq ! ((5 , None) , super :: and ((2 , Some (2)) , (3 , None))) ; assert_eq ! ((5 , None) , super :: and ((2 , None) , (3 , Some (3)))) ; assert_eq ! ((5 , None) , super :: and ((2 , None) , (3 , None))) ; } # [test] fn or () { assert_eq ! ((2 , Some (3)) , super :: or ((2 , Some (2)) , (3 , Some (3)))) ; assert_eq ! ((2 , None) , super :: or ((2 , Some (2)) , (3 , None))) ; assert_eq ! ((2 , None) , super :: or ((2 , None) , (3 , Some (3)))) ; assert_eq ! ((2 , None) , super :: or ((2 , None) , (3 , None))) ; } # [test] fn and_all () { assert_eq ! ((0 , Some (0)) , super :: and_all (& [])) ; assert_eq ! ((7 , Some (7)) , super :: and_all (& [(1 , Some (1)) , (2 , Some (2)) , (4 , Some (4))])) ; assert_eq ! ((7 , None) , super :: and_all (& [(1 , Some (1)) , (2 , Some (2)) , (4 , None)])) ; assert_eq ! ((7 , None) , super :: and_all (& [(1 , Some (1)) , (2 , None) , (4 , Some (4))])) ; assert_eq ! ((7 , None) , super :: and_all (& [(1 , None) , (2 , Some (2)) , (4 , Some (4))])) ; } # [test] fn or_all () { assert_eq ! ((0 , Some (0)) , super :: or_all (& [])) ; assert_eq ! ((1 , Some (4)) , super :: or_all (& [(1 , Some (1)) , (2 , Some (2)) , (4 , Some (4))])) ; assert_eq ! ((1 , None) , super :: or_all (& [(1 , Some (1)) , (2 , Some (2)) , (4 , None)])) ; assert_eq ! ((1 , None) , super :: or_all (& [(1 , Some (1)) , (2 , None) , (4 , Some (4))])) ; assert_eq ! ((1 , None) , super :: or_all (& [(1 , None) , (2 , Some (2)) , (4 , Some (4))])) ; } }
};
}
