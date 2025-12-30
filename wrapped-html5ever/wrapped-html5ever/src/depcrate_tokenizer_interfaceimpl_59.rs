// Generated macro for impl_59 (impl)
macro_rules! Depcrate_tokenizer_interfaceimpl_59 {
() => {
// Module: crate::tokenizer::interface
// Provides: {"impl_59"}
// Dependencies: {}
impl Tag { # [doc = " Are the tags equivalent when we don't care about attribute order?"] # [doc = " Also ignores the self-closing flag."] pub fn equiv_modulo_attr_order (& self , other : & Tag) -> bool { if (self . kind != other . kind) || (self . name != other . name) { return false ; } let mut self_attrs = self . attrs . clone () ; let mut other_attrs = other . attrs . clone () ; self_attrs . sort () ; other_attrs . sort () ; self_attrs == other_attrs } }
};
}
