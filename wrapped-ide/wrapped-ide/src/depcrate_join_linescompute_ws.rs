// Generated macro for compute_ws (function)
macro_rules! Depcrate_join_linescompute_ws {
() => {
// Module: crate::join_lines
// Provides: {"compute_ws"}
// Dependencies: {}
fn compute_ws (left : SyntaxKind , right : SyntaxKind) -> & 'static str { match left { T ! ['('] | T ! ['['] => return "" , T ! ['{'] => { if let USE_TREE = right { return "" ; } } _ => () , } match right { T ! [')'] | T ! [']'] => return "" , T ! ['}'] => { if let USE_TREE = left { return "" ; } } T ! [.] => return "" , _ => () , } " " }
};
}
