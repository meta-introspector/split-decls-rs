// Generated macro for compile_pattern (function)
macro_rules! Depcrate_globcompile_pattern {
() => {
// Module: crate::glob
// Provides: {"compile_pattern"}
// Dependencies: {}
fn compile_pattern (pattern : & RelativePath) -> Vec < Component < '_ > > { let mut output = Vec :: new () ; for c in pattern . components () { output . push (match c { relative_path :: Component :: CurDir => continue , relative_path :: Component :: ParentDir => Component :: ParentDir , relative_path :: Component :: Normal ("**") => Component :: StarStar , relative_path :: Component :: Normal (normal) => { let fragment = Fragment :: parse (normal) ; if let Some (normal) = fragment . as_literal () { Component :: Normal (normal) } else { Component :: Fragment (fragment) } } }) ; } output }
};
}
