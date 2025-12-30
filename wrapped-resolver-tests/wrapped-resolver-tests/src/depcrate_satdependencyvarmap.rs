// Generated macro for DependencyVarMap (type)
macro_rules! Depcrate_satDependencyVarMap {
() => {
// Module: crate::sat
// Provides: {"DependencyVarMap"}
// Dependencies: {}
type DependencyVarMap < 'a > = HashMap < InternedString , HashMap < (DepKind , Option < & 'a Platform >) , varisat :: Var > > ;
};
}
