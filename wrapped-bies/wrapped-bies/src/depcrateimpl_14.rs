// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl Writeable for BiesString < '_ > { fn write_to < W : std :: fmt :: Write + ? Sized > (& self , sink : & mut W) -> std :: fmt :: Result { let mut write_bies_word = | i : usize , j : usize | -> fmt :: Result { if i == j - 1 { sink . write_char ('s') ? ; } else { sink . write_char ('b') ? ; for _ in (i + 1) .. (j - 1) { sink . write_char ('i') ? ; } sink . write_char ('e') ? ; } Ok (()) } ; let mut i = 0 ; for j in self . 0 . breakpoints . iter () . copied () { write_bies_word (i , j) ? ; i = j ; } write_bies_word (i , self . 0 . length) ? ; Ok (()) } fn writeable_length_hint (& self) -> writeable :: LengthHint { LengthHint :: exact (self . 0 . length) } }
};
}
