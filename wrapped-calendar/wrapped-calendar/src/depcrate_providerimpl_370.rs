// Generated macro for impl_370 (impl)
macro_rules! Depcrate_providerimpl_370 {
() => {
// Module: crate::provider
// Provides: {"impl_370"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl serde :: Serialize for WeekdaySet { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { if serializer . is_human_readable () { use serde :: ser :: SerializeSeq ; let mut seq = serializer . serialize_seq (None) ? ; for day in crate :: week :: WeekdaySetIterator :: new (Weekday :: Monday , * self) { seq . serialize_element (& day) ? ; } seq . end () } else { self . 0 . serialize (serializer) } } }
};
}
