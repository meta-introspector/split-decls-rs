// Generated macro for impl_278 (impl)
macro_rules! Depcrate_serimpl_278 {
() => {
// Module: crate::ser
// Provides: {"impl_278"}
// Dependencies: {}
# [cfg (feature = "indexmap")] impl < K , V , S > BorshSerialize for indexmap :: IndexMap < K , V , S > where K : BorshSerialize , V : BorshSerialize , { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { check_zst :: < K > () ? ; let iterator = self . iter () ; u32 :: try_from (iterator . len ()) . map_err (| _ | ErrorKind :: InvalidData) ? . serialize (writer) ? ; for (key , value) in iterator { key . serialize (writer) ? ; value . serialize (writer) ? ; } Ok (()) } }
};
}
