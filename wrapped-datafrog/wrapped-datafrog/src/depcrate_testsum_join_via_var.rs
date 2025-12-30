// Generated macro for sum_join_via_var (function)
macro_rules! Depcrate_testsum_join_via_var {
() => {
// Module: crate::test
// Provides: {"sum_join_via_var"}
// Dependencies: {}
# [doc = " Computes a join where the values are summed -- uses iteration"] # [doc = " variables (the original datafrog technique)."] fn sum_join_via_var (input1_slice : & [(u32 , u32)] , input2_slice : & [(u32 , u32)] ,) -> Relation < (u32 , u32) > { let mut iteration = Iteration :: new () ; let input1 = iteration . variable :: < (u32 , u32) > ("input1") ; input1 . extend (input1_slice) ; let input2 = iteration . variable :: < (u32 , u32) > ("input1") ; input2 . extend (input2_slice) ; let output = iteration . variable :: < (u32 , u32) > ("output") ; while iteration . changed () { output . from_join (& input1 , & input2 , | & k1 , & v1 , & v2 | (k1 , v1 * 100 + v2)) ; } output . complete () }
};
}
