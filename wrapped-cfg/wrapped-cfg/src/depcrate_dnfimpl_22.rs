// Generated macro for impl_22 (impl)
macro_rules! Depcrate_dnfimpl_22 {
() => {
// Module: crate::dnf
// Provides: {"impl_22"}
// Dependencies: {}
impl Conjunction { fn new (parts : Box < [CfgExpr] >) -> Self { let mut literals = Vec :: new () ; for part in parts . into_vec () { match part { CfgExpr :: Invalid | CfgExpr :: Atom (_) | CfgExpr :: Not (_) => { literals . push (Literal :: new (part)) ; } CfgExpr :: All (conj) => { literals . extend (Conjunction :: new (conj) . literals) ; } CfgExpr :: Any (_) => unreachable ! ("disjunction in conjunction") , } } Self { literals } } }
};
}
