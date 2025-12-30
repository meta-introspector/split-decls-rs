// Generated macro for impl_18 (impl)
macro_rules! Depcrate_nodeimpl_18 {
() => {
// Module: crate::node
// Provides: {"impl_18"}
// Dependencies: {}
impl Node { pub fn new (obid : String , name : String) -> Self { let mut upper = true ; let mut symb = String :: new () ; for c in name . chars () { match upper { false => symb . push (c) , true => symb . push (c . to_ascii_uppercase ()) , } match c { '-' => upper = true , _ => upper = false , } } let symb = symb . to_case (Case :: UpperSnake) ; let symb = Ident :: new (& symb , Span :: call_site ()) ; Self { obid , name , symb } } pub fn name (& self) -> & str { & self . name } pub fn symbol (& self) -> & Ident { & self . symb } pub fn definition (& self) -> TokenStream { let obid = self . obid . replace (' ' , "") ; let symb = & self . symb ; quote ! { pub const # symb : crate :: ObjectIdentifier = crate :: ObjectIdentifier :: new_unwrap (# obid) ; } } }
};
}
