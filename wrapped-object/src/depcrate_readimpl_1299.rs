// Generated macro for impl_1299 (impl)
macro_rules! Depcrate_readimpl_1299 {
() => {
// Module: crate::read
// Provides: {"impl_1299"}
// Dependencies: {}
impl < 'data > SymbolMapName < 'data > { # [doc = " Construct a `SymbolMapName`."] pub fn new (address : u64 , name : & 'data str) -> Self { SymbolMapName { address , name } } # [doc = " The symbol address."] # [inline] pub fn address (& self) -> u64 { self . address } # [doc = " The symbol name."] # [inline] pub fn name (& self) -> & 'data str { self . name } }
};
}
