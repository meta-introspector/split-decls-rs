// Generated macro for impl_35 (impl)
macro_rules! Depcrate_serdeimpl_35 {
() => {
// Module: crate::serde
// Provides: {"impl_35"}
// Dependencies: {}
impl Serialize for Level { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { match * self { Level :: Error => serializer . serialize_unit_variant ("Level" , 0 , "ERROR") , Level :: Warn => serializer . serialize_unit_variant ("Level" , 1 , "WARN") , Level :: Info => serializer . serialize_unit_variant ("Level" , 2 , "INFO") , Level :: Debug => serializer . serialize_unit_variant ("Level" , 3 , "DEBUG") , Level :: Trace => serializer . serialize_unit_variant ("Level" , 4 , "TRACE") , } } }
};
}
