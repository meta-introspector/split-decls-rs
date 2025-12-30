// Generated macro for impl_14 (impl)
macro_rules! Depcrate_deimpl_14 {
() => {
// Module: crate::de
// Provides: {"impl_14"}
// Dependencies: {}
impl TakeExt for [u8] { fn take_one (& self) -> Result < (u8 , & [u8]) , Error > { if let Some ((first , rest)) = self . split_first () { Ok ((* first , rest)) } else { Err (Error :: UnexpectedEndOfData) } } fn take_n (& self , n : usize) -> Result < (& [u8] , & [u8]) , Error > { if self . len () < n { return Err (Error :: UnexpectedEndOfData) ; } Ok (self . split_at (n)) } }
};
}
