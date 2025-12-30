// Generated macro for iter_test (function)
macro_rules! Depcrateiter_test {
() => {
// Module: crate
// Provides: {"iter_test"}
// Dependencies: {}
# [test] fn iter_test () { use alloc :: vec :: Vec ; let chars : Vec < _ > = AsciiCharsIter :: new ("🄏中国") . flatten () . collect () ; assert_eq ! (& chars , & ["NonCommercial" , "Zhong " , "Guo"]) ; let chars : Vec < _ > = "中国x🅶" . ascii_chars () . flatten () . collect () ; assert_eq ! (& chars , & ["Zhong " , "Guo " , "x" , "G"]) ; let chars : Vec < _ > = "☃中 国" . ascii_chars () . flatten () . collect () ; assert_eq ! (& chars , & ["snowman " , "Zhong" , " " , "Guo"]) ; }
};
}
