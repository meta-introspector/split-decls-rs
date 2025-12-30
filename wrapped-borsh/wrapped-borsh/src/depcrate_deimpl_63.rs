// Generated macro for impl_63 (impl)
macro_rules! Depcrate_deimpl_63 {
() => {
// Module: crate::de
// Provides: {"impl_63"}
// Dependencies: {}
# [cfg (feature = "indexmap")] impl < T , S > BorshDeserialize for indexmap :: IndexSet < T , S > where T : BorshDeserialize + Eq + core :: hash :: Hash , S : core :: hash :: BuildHasher + Default , { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { check_zst :: < T > () ? ; let vec = < Vec < T > > :: deserialize_reader (reader) ? ; Ok (vec . into_iter () . collect :: < indexmap :: IndexSet < T , S > > ()) } }
};
}
