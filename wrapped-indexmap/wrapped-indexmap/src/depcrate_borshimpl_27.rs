// Generated macro for impl_27 (impl)
macro_rules! Depcrate_borshimpl_27 {
() => {
// Module: crate::borsh
// Provides: {"impl_27"}
// Dependencies: {}
# [doc = " <div class=\"stab deprecated\"><span class=\"emoji\">👎</span><span>Deprecated: use borsh's <code>indexmap</code> feature instead.</span></div>"] impl < T , S > BorshDeserialize for IndexSet < T , S > where T : BorshDeserialize + Eq + Hash , S : BuildHasher + Default , { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { check_zst :: < T > () ? ; let vec = < Vec < T > > :: deserialize_reader (reader) ? ; Ok (vec . into_iter () . collect :: < IndexSet < T , S > > ()) } }
};
}
