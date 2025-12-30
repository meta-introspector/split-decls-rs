// Generated macro for impl_15 (impl)
macro_rules! Depcrate_entryimpl_15 {
() => {
// Module: crate::entry
// Provides: {"impl_15"}
// Dependencies: {}
impl Entry < '_ > { fn fill_buf (& mut self) -> std :: io :: Result < & [u8] > { if self . parent . pos >= self . parent . filled { let mut u16_buf = [0 ; 2] ; self . parent . read . read_exact (& mut u16_buf) ? ; let nb = u16 :: from_le_bytes (u16_buf) as usize ; if nb != 0 { self . parent . read . read_exact (& mut self . parent . buf [.. nb]) ? ; } self . parent . filled = nb ; self . parent . pos = 0 ; } Ok (& self . parent . buf [self . parent . pos .. self . parent . filled]) } }
};
}
