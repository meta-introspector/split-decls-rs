// Generated macro for impl_113 (impl)
macro_rules! Depcrate_item_typeimpl_113 {
() => {
// Module: crate::item_type
// Provides: {"impl_113"}
// Dependencies: {}
impl HelperAttributeForDefault { fn from_attrs (attrs : & [Attribute]) -> Result < Option < Self > > { if let Some (args) = parse_single :: < ArgsForDefault > (attrs , "default") ? { let value = if args . value == parse_quote ! (_) { None } else { Some (args . value) } ; Ok (Some (Self { value , bounds : Bounds :: from (& args . bound) , })) } else { Ok (None) } } fn value (& self , ty : & Type) -> Option < TokenStream > { fn need_into (e : & Expr) -> bool { matches ! (e , Expr :: Lit (ExprLit { lit : Lit :: Str (_) , .. }) | Expr :: Path (_)) } if let Some (e) = & self . value { return Some (if need_into (e) { quote ! (:: core :: convert :: Into ::<# ty >:: into (# e)) } else { quote ! (# e) }) ; } None } }
};
}
