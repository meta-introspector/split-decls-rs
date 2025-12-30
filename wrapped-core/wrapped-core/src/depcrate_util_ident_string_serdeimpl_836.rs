// Generated macro for impl_836 (impl)
macro_rules! Depcrate_util_ident_string_serdeimpl_836 {
() => {
// Module: crate::util::ident_string::serde
// Provides: {"impl_836"}
// Dependencies: {}
impl < 'de > serde :: de :: Visitor < 'de > for IdentStringVisitor { type Value = IdentString ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (formatter , "a valid ident") } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : serde :: de :: Error , { let ident : Ident = syn :: parse_str (v) . map_err (serde :: de :: Error :: custom) ? ; Ok (IdentString :: new (ident)) } }
};
}
