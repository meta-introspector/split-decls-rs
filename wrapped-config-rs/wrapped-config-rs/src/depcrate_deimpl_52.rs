// Generated macro for impl_52 (impl)
macro_rules! Depcrate_deimpl_52 {
() => {
// Module: crate::de
// Provides: {"impl_52"}
// Dependencies: {}
impl EnumAccess { fn variant_deserializer (& self , name : & str) -> Result < StrDeserializer < '_ > > { self . variants . iter () . find (| & & s | s == name) . map (| & s | StrDeserializer (s)) . ok_or_else (| | self . no_constructor_error (name)) } fn table_deserializer (& self , table : & Table) -> Result < StrDeserializer < '_ > > { if table . len () == 1 { self . variant_deserializer (table . iter () . next () . unwrap () . 0) } else { Err (self . structural_error ()) } } fn no_constructor_error (& self , supposed_variant : & str) -> ConfigError { ConfigError :: Message (format ! ("enum {} does not have variant constructor {}" , self . name , supposed_variant)) } fn structural_error (& self) -> ConfigError { ConfigError :: Message (format ! ("value of enum {} should be represented by either string or table with exactly one key" , self . name)) } }
};
}
