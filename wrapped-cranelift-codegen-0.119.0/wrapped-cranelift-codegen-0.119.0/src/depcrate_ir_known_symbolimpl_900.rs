// Generated macro for impl_900 (impl)
macro_rules! Depcrate_ir_known_symbolimpl_900 {
() => {
// Module: crate::ir::known_symbol
// Provides: {"impl_900"}
// Dependencies: {}
impl FromStr for KnownSymbol { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "ElfGlobalOffsetTable" => Ok (Self :: ElfGlobalOffsetTable) , "CoffTlsIndex" => Ok (Self :: CoffTlsIndex) , _ => Err (()) , } } }
};
}
