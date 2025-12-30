// Generated macro for sum_join_via_relation (function)
macro_rules! Depcrate_testsum_join_via_relation {
() => {
// Module: crate::test
// Provides: {"sum_join_via_relation"}
// Dependencies: {}
# [doc = " Computes a join where the values are summed -- uses iteration"] # [doc = " variables (the original datafrog technique)."] fn sum_join_via_relation (input1_slice : & [(u32 , u32)] , input2_slice : & [(u32 , u32)] ,) -> Relation < (u32 , u32) > { let input1 : Relation < _ > = input1_slice . iter () . collect () ; let input2 : Relation < _ > = input2_slice . iter () . collect () ; Relation :: from_join (& input1 , & input2 , | & k1 , & v1 , & v2 | (k1 , v1 * 100 + v2)) }
};
}
