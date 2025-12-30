// Generated macro for run_tests (macro)
macro_rules! Depcrate_macrosrun_tests {
() => {
// Module: crate::macros
// Provides: {"run_tests"}
// Dependencies: {}
macro_rules ! run_tests { ($ dst : expr , $ ($ testfn : ident) ,* $ (,) ?) => { use crossterm :: { queue , style , terminal , cursor } ; $ (queue ! ($ dst , style :: ResetColor , terminal :: Clear (terminal :: ClearType :: All) , cursor :: MoveTo (1 , 1) , cursor :: Show , cursor :: EnableBlinking) ?; $ testfn ($ dst) ?; match $ crate :: read_char () { Ok ('q') => return Ok (()) , Err (e) => return Err (e) , _ => { } , } ;) * } }
};
}
