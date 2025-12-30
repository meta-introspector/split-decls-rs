// Generated macro for impl_16 (impl)
macro_rules! Depcrate_compactdecimal_formatimpl_16 {
() => {
// Module: crate::compactdecimal::format
// Provides: {"impl_16"}
// Dependencies: {}
impl Writeable for FormattedCompactDecimal < '_ > { fn write_to < W > (& self , sink : & mut W) -> core :: result :: Result < () , core :: fmt :: Error > where W : core :: fmt :: Write + ? Sized , { if self . value . exponent () == 0 { self . formatter . decimal_formatter . format (self . value . significand ()) . write_to (sink) } else { let plural_map = self . plural_map . as_ref () . ok_or (core :: fmt :: Error) ? ; let chosen_pattern = (| | { if self . value . significand () == & Decimal :: from (1) { if let Some (pattern) = plural_map . get1 (& Count :: Explicit1) { return Some (pattern) ; } } let plural_category = self . formatter . plural_rules . category_for (self . value . significand ()) ; plural_map . get1 (& plural_category . into ()) . or_else (| | plural_map . get1 (& Count :: Other)) }) () . ok_or (core :: fmt :: Error) ? ; match chosen_pattern . index { u8 :: MAX => sink . write_str (& chosen_pattern . literal_text) , _ => { let i = usize :: from (chosen_pattern . index) ; sink . write_str (chosen_pattern . literal_text . get (.. i) . ok_or (core :: fmt :: Error) ? ,) ? ; self . formatter . decimal_formatter . format (self . value . significand ()) . write_to (sink) ? ; sink . write_str (chosen_pattern . literal_text . get (i ..) . ok_or (core :: fmt :: Error) ? ,) } } } } }
};
}
