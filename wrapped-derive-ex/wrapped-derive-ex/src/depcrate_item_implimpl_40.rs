// Generated macro for impl_40 (impl)
macro_rules! Depcrate_item_implimpl_40 {
() => {
// Module: crate::item_impl
// Provides: {"impl_40"}
// Dependencies: {}
impl Op { fn new (op : BinaryOp , form : OpForm) -> Self { Self { op , form } } fn from_str (mut s : & str) -> Option < Self > { let suffix = "Assign" ; let mut form = OpForm :: Binary ; if s . ends_with (suffix) { s = & s [.. s . len () - suffix . len ()] ; form = OpForm :: Assign ; } Some (Self :: new (BinaryOp :: from_str (s) ? , form)) } fn from_ident (s : & Ident) -> Result < Self > { let s = s . to_string () ; if let Some (op) = Self :: from_str (& s) { Ok (op) } else { bail ! (s . span () , "`{}` is not supported for `#[derive_ex]`" , s) ; } } fn to_func_ident (self) -> Ident { let name = self . op . to_func_name () ; let assign = if self . form == OpForm :: Assign { "_assign" } else { "" } ; Ident :: new (& format ! ("{name}{assign}") , Span :: call_site ()) } fn to_trait_ident (self) -> Ident { Ident :: new (& self . to_string () , Span :: call_site ()) } fn to_trait_path (self) -> Path { let ident = self . to_trait_ident () ; parse_quote ! (:: core :: ops ::# ident) } }
};
}
