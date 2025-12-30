// Generated macro for impl_87 (impl)
macro_rules! Depcrate_typesimpl_87 {
() => {
// Module: crate::types
// Provides: {"impl_87"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for MacroMatcher { fn deserialize < D > (deser : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { # [derive (Deserialize)] # [serde (field_identifier , rename_all = "lowercase")] enum Field { Name , Brace , } struct MacVisitor ; impl < 'de > Visitor < 'de > for MacVisitor { type Value = MacroMatcher ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("struct MacroMatcher") } fn visit_map < V > (self , mut map : V) -> Result < Self :: Value , V :: Error > where V : de :: MapAccess < 'de > , { let mut name = None ; let mut brace : Option < char > = None ; while let Some (key) = map . next_key () ? { match key { Field :: Name => { if name . is_some () { return Err (de :: Error :: duplicate_field ("name")) ; } name = Some (map . next_value () ?) ; } , Field :: Brace => { if brace . is_some () { return Err (de :: Error :: duplicate_field ("brace")) ; } brace = Some (map . next_value () ?) ; } , } } let name = name . ok_or_else (| | de :: Error :: missing_field ("name")) ? ; let brace = brace . ok_or_else (| | de :: Error :: missing_field ("brace")) ? ; Ok (MacroMatcher { name , braces : [('(' , ')') , ('{' , '}') , ('[' , ']')] . into_iter () . find (| b | b . 0 == brace) . map (| (o , c) | (o . to_owned () , c . to_owned ())) . ok_or_else (| | de :: Error :: custom (format ! ("expected one of `(`, `{{`, `[` found `{brace}`"))) ? , }) } } const FIELDS : & [& str] = & ["name" , "brace"] ; deser . deserialize_struct ("MacroMatcher" , FIELDS , MacVisitor) } }
};
}
