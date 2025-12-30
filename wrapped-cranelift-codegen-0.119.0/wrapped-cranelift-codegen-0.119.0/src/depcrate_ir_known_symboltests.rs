// Generated macro for tests (module)
macro_rules! Depcrate_ir_known_symboltests {
() => {
// Module: crate::ir::known_symbol
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn parsing () { assert_eq ! ("ElfGlobalOffsetTable" . parse () , Ok (KnownSymbol :: ElfGlobalOffsetTable)) ; assert_eq ! ("CoffTlsIndex" . parse () , Ok (KnownSymbol :: CoffTlsIndex)) ; } }
};
}
