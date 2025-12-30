// Generated macro for impl_280 (impl)
macro_rules! Depcrate_serimpl_280 {
() => {
// Module: crate::ser
// Provides: {"impl_280"}
// Dependencies: {}
impl < T > BorshSerialize for LinkedList < T > where T : BorshSerialize , { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { check_zst :: < T > () ? ; writer . write_all (& (u32 :: try_from (self . len ()) . map_err (| _ | ErrorKind :: InvalidData) ?) . to_le_bytes () ,) ? ; for item in self { item . serialize (writer) ? ; } Ok (()) } }
};
}
