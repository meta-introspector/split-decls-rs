// Generated macro for impl_1143 (impl)
macro_rules! Depcrate_unicodeset_parse_parseimpl_1143 {
() => {
// Module: crate::unicodeset_parse::parse
// Provides: {"impl_1143"}
// Dependencies: {}
impl < 'data > MainToken < 'data > { fn from_variable_value (val : VariableValue < 'data >) -> Self { match val { VariableValue :: Char (c) => { MainToken :: Literal (Literal :: CharKind (SingleOrMultiChar :: Single (c))) } VariableValue :: String (s) => { MainToken :: Literal (Literal :: String (s . into_owned ())) } VariableValue :: UnicodeSet (set) => MainToken :: UnicodeSet (set) , } } }
};
}
