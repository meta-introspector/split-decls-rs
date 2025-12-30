// Generated macro for impl_277 (impl)
macro_rules! Depcrate_serimpl_277 {
() => {
// Module: crate::ser
// Provides: {"impl_277"}
// Dependencies: {}
# [cfg (feature = "indexmap")] impl < T , S > BorshSerialize for indexmap :: IndexSet < T , S > where T : BorshSerialize , { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { check_zst :: < T > () ? ; let iterator = self . iter () ; u32 :: try_from (iterator . len ()) . map_err (| _ | ErrorKind :: InvalidData) ? . serialize (writer) ? ; for item in iterator { item . serialize (writer) ? ; } Ok (()) } }
};
}
