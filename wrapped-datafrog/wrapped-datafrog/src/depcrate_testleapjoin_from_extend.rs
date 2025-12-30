// Generated macro for leapjoin_from_extend (function)
macro_rules! Depcrate_testleapjoin_from_extend {
() => {
// Module: crate::test
// Provides: {"leapjoin_from_extend"}
// Dependencies: {}
# [doc = " Test that `from_leapjoin` matches against the tuples from an"] # [doc = " `extend` that precedes first iteration."] # [doc = ""] # [doc = " This was always true, but wasn't immediately obvious to me until I"] # [doc = " re-read the code more carefully. -nikomatsakis"] # [test] fn leapjoin_from_extend () { let doubles : Relation < (u32 , u32) > = (0 .. 10) . map (| i | (i , i * 2)) . collect () ; let mut iteration = Iteration :: new () ; let variable = iteration . variable :: < (u32 , u32) > ("variable") ; variable . extend (Some ((2 , 2))) ; while iteration . changed () { variable . from_leapjoin (& variable , doubles . extend_with (| & (i , _) | i) , | & (i , _) , & j | (i , j) ,) ; } let variable = variable . complete () ; assert_eq ! (variable . elements , vec ! [(2 , 2) , (2 , 4)]) ; }
};
}
