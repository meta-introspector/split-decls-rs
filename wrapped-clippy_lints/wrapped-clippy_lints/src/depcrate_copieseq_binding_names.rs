// Generated macro for eq_binding_names (function)
macro_rules! Depcrate_copieseq_binding_names {
() => {
// Module: crate::copies
// Provides: {"eq_binding_names"}
// Dependencies: {}
# [doc = " If the statement is a local, checks if the bound names match the expected list of names."] fn eq_binding_names (s : & Stmt < '_ > , names : & [(HirId , Symbol)]) -> bool { if let StmtKind :: Let (l) = s . kind { let mut i = 0usize ; let mut res = true ; l . pat . each_binding_or_first (& mut | _ , _ , _ , name | { if names . get (i) . is_some_and (| & (_ , n) | n == name . name) { i += 1 ; } else { res = false ; } }) ; res && i == names . len () } else { false } }
};
}
