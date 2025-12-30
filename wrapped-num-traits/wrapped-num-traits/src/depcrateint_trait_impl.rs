// Generated macro for int_trait_impl (macro)
macro_rules! Depcrateint_trait_impl {
() => {
// Module: crate
// Provides: {"int_trait_impl"}
// Dependencies: {}
macro_rules ! int_trait_impl { ($ name : ident for $ ($ t : ty) *) => ($ (impl $ name for $ t { type FromStrRadixErr = :: core :: num :: ParseIntError ; # [inline] fn from_str_radix (s : & str , radix : u32) -> Result < Self , :: core :: num :: ParseIntError > { <$ t >:: from_str_radix (s , radix) } }) *) }
};
}
