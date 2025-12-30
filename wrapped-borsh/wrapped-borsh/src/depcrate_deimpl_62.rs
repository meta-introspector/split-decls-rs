// Generated macro for impl_62 (impl)
macro_rules! Depcrate_deimpl_62 {
() => {
// Module: crate::de
// Provides: {"impl_62"}
// Dependencies: {}
# [cfg (feature = "indexmap")] impl < K , V , S > BorshDeserialize for indexmap :: IndexMap < K , V , S > where K : BorshDeserialize + Eq + core :: hash :: Hash , V : BorshDeserialize , S : core :: hash :: BuildHasher + Default , { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { check_zst :: < K > () ? ; let vec = < Vec < (K , V) > > :: deserialize_reader (reader) ? ; Ok (vec . into_iter () . collect :: < indexmap :: IndexMap < K , V , S > > ()) } }
};
}
