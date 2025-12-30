// Generated macro for impl_24 (impl)
macro_rules! Depcrate_borshimpl_24 {
() => {
// Module: crate::borsh
// Provides: {"impl_24"}
// Dependencies: {}
# [doc = " <div class=\"stab deprecated\"><span class=\"emoji\">👎</span><span>Deprecated: use borsh's <code>indexmap</code> feature instead.</span></div>"] impl < K , V , S > BorshSerialize for IndexMap < K , V , S > where K : BorshSerialize , V : BorshSerialize , { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { check_zst :: < K > () ? ; let iterator = self . iter () ; u32 :: try_from (iterator . len ()) . map_err (| _ | ErrorKind :: InvalidData) ? . serialize (writer) ? ; for (key , value) in iterator { key . serialize (writer) ? ; value . serialize (writer) ? ; } Ok (()) } }
};
}
