// Generated macro for run_tests (macro)
macro_rules! Depcrate_macrosrun_tests {
() => {
// Module: crate::macros
// Provides: {"run_tests"}
// Dependencies: {}
macro_rules ! run_tests { ($ dst : expr_2021 , $ ($ testfn : ident) ,* $ (,) ?) => { $ (:: anes :: queue ! ($ dst , anes :: ResetAttributes , anes :: ClearBuffer :: All , anes :: MoveCursorTo (1 , 1) , anes :: ShowCursor , anes :: EnableCursorBlinking ,) ?; $ testfn ($ dst) ?; match $ crate :: read_char () { Ok ('q') => return Ok (()) , Err (e) => return Err (e) , _ => { } , } ;) * } }
};
}
