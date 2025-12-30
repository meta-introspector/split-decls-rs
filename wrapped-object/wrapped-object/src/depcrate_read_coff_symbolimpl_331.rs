// Generated macro for impl_331 (impl)
macro_rules! Depcrate_read_coff_symbolimpl_331 {
() => {
// Module: crate::read::coff::symbol
// Provides: {"impl_331"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > CoffSymbol < 'data , 'file , R , Coff > { # [inline] # [doc = " Get the raw `ImageSymbol` struct."] # [deprecated (note = "Use `coff_symbol` instead")] pub fn raw_symbol (& self) -> & 'data Coff :: ImageSymbol { self . symbol } # [doc = " Get the raw `ImageSymbol` struct."] pub fn coff_symbol (& self) -> & 'data Coff :: ImageSymbol { self . symbol } }
};
}
