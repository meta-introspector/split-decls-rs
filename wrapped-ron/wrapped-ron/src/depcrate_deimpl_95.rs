// Generated macro for impl_95 (impl)
macro_rules! Depcrate_deimpl_95 {
() => {
// Module: crate::de
// Provides: {"impl_95"}
// Dependencies: {}
impl < 'a , 'de > CommaSeparated < 'a , 'de > { fn new (terminator : Terminator , de : & 'a mut Deserializer < 'de >) -> Self { CommaSeparated { de , terminator , had_comma : true , inside_internally_tagged_enum : false , } } fn has_element (& mut self) -> Result < bool > { self . de . parser . skip_ws () ? ; match (self . had_comma , ! self . de . parser . check_char (self . terminator . as_char ()) ,) { (true , has_element) => Ok (has_element) , (false , false) => Ok (false) , (false , true) => Err (Error :: ExpectedComma) , } } }
};
}
