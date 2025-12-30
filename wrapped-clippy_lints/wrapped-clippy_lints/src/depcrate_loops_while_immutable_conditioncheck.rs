// Generated macro for check (function)
macro_rules! Depcrate_loops_while_immutable_conditioncheck {
() => {
// Module: crate::loops::while_immutable_condition
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , cond : & 'tcx Expr < '_ > , expr : & 'tcx Expr < '_ >) { if ConstEvalCtxt :: new (cx) . eval (cond) . is_some () { return ; } let mut var_visitor = VarCollectorVisitor { cx , ids : HirIdSet :: default () , def_ids : DefIdMap :: default () , } ; if var_visitor . visit_expr (cond) . is_break () { return ; } let used_in_condition = & var_visitor . ids ; let mutated_in_body = mutated_variables (expr , cx) ; let mutated_in_condition = mutated_variables (cond , cx) ; let no_cond_variable_mutated = if let (Some (used_mutably_body) , Some (used_mutably_cond)) = (mutated_in_body , mutated_in_condition) { used_in_condition . is_disjoint (& used_mutably_body) && used_in_condition . is_disjoint (& used_mutably_cond) } else { return ; } ; let mutable_static_in_cond = var_visitor . def_ids . items () . any (| (_ , v) | * v) ; let mut has_break_or_return_visitor = HasBreakOrReturnVisitor ; let has_break_or_return = has_break_or_return_visitor . visit_expr (expr) . is_break () ; if no_cond_variable_mutated && ! mutable_static_in_cond { span_lint_and_then (cx , WHILE_IMMUTABLE_CONDITION , cond . span , "variables in the condition are not mutated in the loop body" , | diag | { diag . note ("this may lead to an infinite or to a never running loop") ; if has_break_or_return { diag . note ("this loop contains `return`s or `break`s") ; diag . help ("rewrite it as `if cond { loop { } }`") ; } } ,) ; } }
};
}
