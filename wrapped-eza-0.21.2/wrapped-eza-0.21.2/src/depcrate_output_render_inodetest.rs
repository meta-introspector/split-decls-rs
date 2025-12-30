// Generated macro for test (module)
macro_rules! Depcrate_output_render_inodetest {
() => {
// Module: crate::output::render::inode
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] pub mod test { use crate :: fs :: fields as f ; use crate :: output :: cell :: TextCell ; use nu_ansi_term :: Color :: * ; # [test] fn blocklessness () { let io = f :: Inode (1_414_213) ; let expected = TextCell :: paint_str (Cyan . underline () , "1414213") ; assert_eq ! (expected , io . render (Cyan . underline ())) ; } }
};
}
