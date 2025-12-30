// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl < 'a > BitStream < 'a > { fn new (bytes : & 'a [u8] , state : BitState) -> BitStream < 'a > { BitStream { bytes : bytes . iter () , used : 0 , state : state } } fn use_byte (& mut self) -> bool { match self . bytes . next () { Some (& b) => { self . state . v |= (b as u32) << self . state . n ; self . state . n += 8 ; self . used += 1 ; true } None => false } } fn need (& mut self , n : u8) -> bool { if self . state . n < n { if ! self . use_byte () { return false ; } if n > 8 && self . state . n < n { if n > 16 { abort () ; } if ! self . use_byte () { return false ; } } } true } fn take16 (& mut self , n : u8) -> Option < u16 > { if self . need (n) { self . state . n -= n ; let v = self . state . v & ((1 << n) - 1) ; self . state . v >>= n ; Some (v as u16) } else { None } } fn take (& mut self , n : u8) -> Option < u8 > { if n > 8 { abort () ; } self . take16 (n) . map (| v : u16 | v as u8) } fn fill (& mut self) -> BitState { while self . state . n + 8 <= 32 && self . use_byte () { } self . state } }
};
}
