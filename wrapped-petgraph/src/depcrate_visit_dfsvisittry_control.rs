// Generated macro for try_control (macro)
macro_rules! Depcrate_visit_dfsvisittry_control {
() => {
// Module: crate::visit::dfsvisit
// Provides: {"try_control"}
// Dependencies: {}
# [doc = " Return if the expression is a break value, execute the provided statement"] # [doc = " if it is a prune value."] macro_rules ! try_control { ($ e : expr , $ p : stmt) => { try_control ! ($ e , $ p , ()) ; } ; ($ e : expr , $ p : stmt , $ q : stmt) => { match $ e { x => { if x . should_break () { return x ; } else if x . should_prune () { $ p } else { $ q } } } } ; }
};
}
