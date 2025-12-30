// Generated macro for impl_282 (impl)
macro_rules! Depcrate_serimpl_282 {
() => {
// Module: crate::ser
// Provides: {"impl_282"}
// Dependencies: {}
impl < K , V > BorshSerialize for BTreeMap < K , V > where K : BorshSerialize , V : BorshSerialize , { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { check_zst :: < K > () ? ; u32 :: try_from (self . len ()) . map_err (| _ | ErrorKind :: InvalidData) ? . serialize (writer) ? ; for (key , value) in self { key . serialize (writer) ? ; value . serialize (writer) ? ; } Ok (()) } }
};
}
