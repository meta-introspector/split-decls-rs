// Generated macro for exprloc2address (function)
macro_rules! Depcrate_elf2tableexprloc2address {
() => {
// Module: crate::elf2table
// Provides: {"exprloc2address"}
// Dependencies: {}
fn exprloc2address < R : gimli :: read :: Reader < Offset = usize > > (encoding : gimli :: Encoding , data : & gimli :: Expression < R > ,) -> Result < u64 , anyhow :: Error > { let mut pc = data . 0 . clone () ; while pc . len () != 0 { if let Ok (gimli :: Operation :: Address { address }) = gimli :: Operation :: parse (& mut pc , encoding) { return Ok (address) ; } } Err (anyhow ! ("`Operation::Address` not found")) }
};
}
