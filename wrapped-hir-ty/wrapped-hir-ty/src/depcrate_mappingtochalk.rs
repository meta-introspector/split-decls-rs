// Generated macro for ToChalk (trait)
macro_rules! Depcrate_mappingToChalk {
() => {
// Module: crate::mapping
// Provides: {"ToChalk"}
// Dependencies: {}
pub trait ToChalk { type Chalk ; fn to_chalk (self , db : & dyn HirDatabase) -> Self :: Chalk ; fn from_chalk (db : & dyn HirDatabase , chalk : Self :: Chalk) -> Self ; }
};
}
