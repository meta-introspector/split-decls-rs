// Generated macro for CratesMap (struct)
macro_rules! Depcrate_inputCratesMap {
() => {
// Module: crate::input
// Provides: {"CratesMap"}
// Dependencies: {}
# [doc = " The mapping from [`UniqueCrateData`] to their [`Crate`] input."] # [derive (Debug , Default)] pub struct CratesMap (DashMap < UniqueCrateData , Crate , BuildHasherDefault < FxHasher > >) ;
};
}
