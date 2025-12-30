// Generated macro for impl_26 (impl)
macro_rules! Depcrate_borshimpl_26 {
() => {
// Module: crate::borsh
// Provides: {"impl_26"}
// Dependencies: {}
# [doc = " <div class=\"stab deprecated\"><span class=\"emoji\">👎</span><span>Deprecated: use borsh's <code>indexmap</code> feature instead.</span></div>"] impl < T , S > BorshSerialize for IndexSet < T , S > where T : BorshSerialize , { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { check_zst :: < T > () ? ; let iterator = self . iter () ; u32 :: try_from (iterator . len ()) . map_err (| _ | ErrorKind :: InvalidData) ? . serialize (writer) ? ; for item in iterator { item . serialize (writer) ? ; } Ok (()) } }
};
}
