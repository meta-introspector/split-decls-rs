// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'a > Iterator for FromHexIter < 'a > { type Item = Result < u8 , FromHexError > ; fn next (& mut self) -> Option < Result < u8 , FromHexError > > { if self . err { return None ; } let mut modulus = 0 ; let mut buf = 0 ; for (idx , byte) in & mut self . iter { buf <<= 4 ; match byte { b'A' ..= b'F' => buf |= byte - b'A' + 10 , b'a' ..= b'f' => buf |= byte - b'a' + 10 , b'0' ..= b'9' => buf |= byte - b'0' , b' ' | b'\r' | b'\n' | b'\t' => { buf >>= 4 ; continue } _ => { let ch = self . inner [idx ..] . chars () . next () . unwrap () ; self . err = true ; return Some (Err (InvalidHexCharacter (ch , idx))) ; } } modulus += 1 ; if modulus == 2 { return Some (Ok (buf)) ; } } if modulus != 0 { self . err = true ; return Some (Err (InvalidHexLength)) ; } None } fn size_hint (& self) -> (usize , Option < usize >) { let (a , b) = self . iter . size_hint () ; (a / 2 , b . map (| b | b / 2)) } }
};
}
