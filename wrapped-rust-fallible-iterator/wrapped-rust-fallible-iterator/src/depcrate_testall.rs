// Generated macro for all (function)
macro_rules! Depcrate_testall {
() => {
// Module: crate::test
// Provides: {"all"}
// Dependencies: {}
# [test] fn all () { assert ! (convert ([0 , 1 , 2 , 3] . iter () . map (Ok ::<& u32 , () >)) . all (|& i | Ok (i < 4)) . unwrap ()) ; assert ! (! convert ([0 , 1 , 2 , 4] . iter () . map (Ok ::<& u32 , () >)) . all (|& i | Ok (i < 4)) . unwrap ()) ; assert ! (convert ([0 , 1 , 2 , 4] . iter () . map (Ok ::<& u32 , () >)) . all (| _ | Err (())) . is_err ()) ; }
};
}
