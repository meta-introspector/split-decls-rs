// Generated macro for InternMap (type)
macro_rules! DepcrateInternMap {
() => {
// Module: crate
// Provides: {"InternMap"}
// Dependencies: {}
pub type InternMap < T > = DashMap < Arc < T > , () , BuildHasherDefault < FxHasher > > ;
};
}
