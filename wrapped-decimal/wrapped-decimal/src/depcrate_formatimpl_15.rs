// Generated macro for impl_15 (impl)
macro_rules! Depcrate_formatimpl_15 {
() => {
// Module: crate::format
// Provides: {"impl_15"}
// Dependencies: {}
impl Writeable for FormattedDecimal < '_ > { fn write_to_parts < W > (& self , w : & mut W) -> core :: result :: Result < () , core :: fmt :: Error > where W : writeable :: PartsWrite + ? Sized , { let affixes = self . get_affixes () ; if let Some ((part , affixes)) = affixes { w . with_part (part , | w | w . write_str (affixes . 0)) ? ; } let range = self . value . absolute . magnitude_range () ; let upper_magnitude = * range . end () ; let mut range = range . rev () ; let mut has_fraction = false ; w . with_part (parts :: INTEGER , | w | { loop { let m = match range . next () { Some (m) if m < 0 => { has_fraction = true ; break Ok (()) ; } Some (m) => m , None => { break Ok (()) ; } } ; # [expect (clippy :: indexing_slicing)] w . write_char (self . digits [self . value . digit_at (m) as usize]) ? ; if grouper :: check (upper_magnitude , m , self . options . grouping_strategy . unwrap_or_default () , self . symbols . grouping_sizes ,) { w . with_part (parts :: GROUP , | w | { w . write_str (self . symbols . grouping_separator ()) }) ? ; } } }) ? ; if has_fraction { w . with_part (parts :: DECIMAL , | w | { w . write_str (self . symbols . decimal_separator ()) }) ? ; w . with_part (parts :: FRACTION , | w | { let mut m = - 1 ; loop { # [expect (clippy :: indexing_slicing)] w . write_char (self . digits [self . value . digit_at (m) as usize]) ? ; m = match range . next () { Some (m) => m , None => { break Ok (()) ; } } ; } }) ? ; } if let Some ((part , affixes)) = affixes { w . with_part (part , | w | w . write_str (affixes . 1)) ? ; } Ok (()) } }
};
}
