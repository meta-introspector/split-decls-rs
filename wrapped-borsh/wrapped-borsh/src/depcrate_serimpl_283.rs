// Generated macro for impl_283 (impl)
macro_rules! Depcrate_serimpl_283 {
() => {
// Module: crate::ser
// Provides: {"impl_283"}
// Dependencies: {}
impl < T > BorshSerialize for BTreeSet < T > where T : BorshSerialize , { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { check_zst :: < T > () ? ; u32 :: try_from (self . len ()) . map_err (| _ | ErrorKind :: InvalidData) ? . serialize (writer) ? ; for item in self { item . serialize (writer) ? ; } Ok (()) } }
};
}
