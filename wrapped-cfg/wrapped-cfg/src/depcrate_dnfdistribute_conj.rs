// Generated macro for distribute_conj (function)
macro_rules! Depcrate_dnfdistribute_conj {
() => {
// Module: crate::dnf
// Provides: {"distribute_conj"}
// Dependencies: {}
# [doc = " Turns a conjunction of expressions into a disjunction of expressions."] fn distribute_conj (conj : & [CfgExpr]) -> Vec < CfgExpr > { fn go (out : & mut Vec < CfgExpr > , with : & mut Vec < CfgExpr > , rest : & [CfgExpr]) { match rest { [head , tail @ ..] => match head { CfgExpr :: Any (disj) => { for part in disj . iter () { with . push (part . clone ()) ; go (out , with , tail) ; with . pop () ; } } _ => { with . push (head . clone ()) ; go (out , with , tail) ; with . pop () ; } } , _ => { out . push (CfgExpr :: All (with . clone () . into_boxed_slice ())) ; } } } let mut out = Vec :: new () ; let mut with = Vec :: new () ; go (& mut out , & mut with , conj) ; out }
};
}
