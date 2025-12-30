// Generated macro for impl_37 (impl)
macro_rules! Depcrate_serdeimpl_37 {
() => {
// Module: crate::serde
// Provides: {"impl_37"}
// Dependencies: {}
impl Serialize for LevelFilter { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { match * self { LevelFilter :: Off => serializer . serialize_unit_variant ("LevelFilter" , 0 , "OFF") , LevelFilter :: Error => serializer . serialize_unit_variant ("LevelFilter" , 1 , "ERROR") , LevelFilter :: Warn => serializer . serialize_unit_variant ("LevelFilter" , 2 , "WARN") , LevelFilter :: Info => serializer . serialize_unit_variant ("LevelFilter" , 3 , "INFO") , LevelFilter :: Debug => serializer . serialize_unit_variant ("LevelFilter" , 4 , "DEBUG") , LevelFilter :: Trace => serializer . serialize_unit_variant ("LevelFilter" , 5 , "TRACE") , } } }
};
}
