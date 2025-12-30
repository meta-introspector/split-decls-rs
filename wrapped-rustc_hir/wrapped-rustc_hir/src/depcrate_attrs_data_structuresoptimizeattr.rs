// Generated macro for OptimizeAttr (enum)
macro_rules! Depcrate_attrs_data_structuresOptimizeAttr {
() => {
// Module: crate::attrs::data_structures
// Provides: {"OptimizeAttr"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , PartialEq , Eq , Default , PrintAttribute)] # [derive (Encodable , Decodable , HashStable_Generic)] pub enum OptimizeAttr { # [doc = " No `#[optimize(..)]` attribute"] # [default] Default , # [doc = " `#[optimize(none)]`"] DoNotOptimize , # [doc = " `#[optimize(speed)]`"] Speed , # [doc = " `#[optimize(size)]`"] Size , }
};
}
