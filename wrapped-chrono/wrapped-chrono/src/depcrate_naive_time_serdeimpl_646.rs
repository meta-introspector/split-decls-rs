// Generated macro for impl_646 (impl)
macro_rules! Depcrate_naive_time_serdeimpl_646 {
() => {
// Module: crate::naive::time::serde
// Provides: {"impl_646"}
// Dependencies: {}
impl ser :: Serialize for NaiveTime { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { serializer . collect_str (& self) } }
};
}
