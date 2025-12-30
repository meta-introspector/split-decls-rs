// Generated macro for impl_291 (impl)
macro_rules! Depcrate_serimpl_291 {
() => {
// Module: crate::ser
// Provides: {"impl_291"}
// Dependencies: {}
impl < T , const N : usize > BorshSerialize for [T ; N] where T : BorshSerialize , { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { if N == 0 { return Ok (()) ; } else if let Some (u8_slice) = T :: u8_slice (self) { writer . write_all (u8_slice) ? ; } else { for el in self . iter () { el . serialize (writer) ? ; } } Ok (()) } }
};
}
