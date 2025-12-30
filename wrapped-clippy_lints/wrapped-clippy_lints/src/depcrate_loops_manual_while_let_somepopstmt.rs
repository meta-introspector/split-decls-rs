// Generated macro for PopStmt (enum)
macro_rules! Depcrate_loops_manual_while_let_somePopStmt {
() => {
// Module: crate::loops::manual_while_let_some
// Provides: {"PopStmt"}
// Dependencies: {}
# [doc = " The kind of statement that the `pop()` call appeared in."] # [doc = ""] # [doc = " Depending on whether the value was assigned to a variable or not changes what pattern"] # [doc = " we use for the suggestion."] # [derive (Copy , Clone)] enum PopStmt < 'hir > { # [doc = " `x.pop().unwrap()` was and assigned to a variable."] # [doc = " The pattern of this local variable will be used and the local statement"] # [doc = " is deleted in the suggestion."] Local (& 'hir Pat < 'hir >) , # [doc = " `x.pop().unwrap()` appeared in an arbitrary expression and was not assigned to a variable."] # [doc = " The suggestion will use some placeholder identifier and the `x.pop().unwrap()` expression"] # [doc = " is replaced with that identifier."] Anonymous , }
};
}
