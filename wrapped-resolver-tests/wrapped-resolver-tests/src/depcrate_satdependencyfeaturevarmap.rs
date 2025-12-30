// Generated macro for DependencyFeatureVarMap (type)
macro_rules! Depcrate_satDependencyFeatureVarMap {
() => {
// Module: crate::sat
// Provides: {"DependencyFeatureVarMap"}
// Dependencies: {}
type DependencyFeatureVarMap < 'a > = HashMap < InternedString , HashMap < (DepKind , Option < & 'a Platform >) , HashMap < InternedString , varisat :: Var > > , > ;
};
}
