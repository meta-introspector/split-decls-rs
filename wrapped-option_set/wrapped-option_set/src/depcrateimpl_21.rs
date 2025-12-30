// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'a > Visitor < 'a > for StrVisitor { type Value = Str < 'a > ; fn expecting (& self , formatter : & mut Formatter) -> fmt :: Result { formatter . write_str ("a string") } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : serde :: de :: Error , { self . visit_string (v . to_owned ()) } fn visit_string < E > (self , v : String) -> Result < Self :: Value , E > where E : serde :: de :: Error , { Ok (Str :: String (v)) } fn visit_borrowed_str < E > (self , v : & 'a str) -> Result < Self :: Value , E > where E : serde :: de :: Error , { Ok (Str :: Str (v)) } }
};
}
