// Generated macro for local_is_initialized (function)
macro_rules! Depcratelocal_is_initialized {
() => {
// Module: crate
// Provides: {"local_is_initialized"}
// Dependencies: {}
# [doc = " Checks if the given local has an initializer or is from something other than a `let` statement"] # [doc = ""] # [doc = " e.g. returns true for `x` in `fn f(x: usize) { .. }` and `let x = 1;` but false for `let x;`"] pub fn local_is_initialized (cx : & LateContext < '_ > , local : HirId) -> bool { for (_ , node) in cx . tcx . hir_parent_iter (local) { match node { Node :: Pat (..) | Node :: PatField (..) => { } , Node :: LetStmt (let_stmt) => return let_stmt . init . is_some () , _ => return true , } } false }
};
}
