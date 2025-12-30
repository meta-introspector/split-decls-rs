// Generated macro for from_chalk (function)
macro_rules! Depcrate_mappingfrom_chalk {
() => {
// Module: crate::mapping
// Provides: {"from_chalk"}
// Dependencies: {}
pub (crate) fn from_chalk < T , ChalkT > (db : & dyn HirDatabase , chalk : ChalkT) -> T where T : ToChalk < Chalk = ChalkT > , { T :: from_chalk (db , chalk) }
};
}
