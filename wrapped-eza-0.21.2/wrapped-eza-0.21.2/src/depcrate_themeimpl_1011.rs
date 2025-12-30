// Generated macro for impl_1011 (impl)
macro_rules! Depcrate_themeimpl_1011 {
() => {
// Module: crate::theme
// Provides: {"impl_1011"}
// Dependencies: {}
impl render :: SizeColours for Theme { fn size (& self , prefix : Option < number_prefix :: Prefix >) -> Style { use number_prefix :: Prefix :: * ; # [rustfmt :: skip] return match prefix { Some (Kilo | Kibi) => self . ui . size . unwrap_or_default () . number_kilo () , Some (Mega | Mebi) => self . ui . size . unwrap_or_default () . number_mega () , Some (Giga | Gibi) => self . ui . size . unwrap_or_default () . number_giga () , Some (_) => self . ui . size . unwrap_or_default () . number_huge () , None => self . ui . size . unwrap_or_default () . number_byte () , } ; } fn unit (& self , prefix : Option < number_prefix :: Prefix >) -> Style { use number_prefix :: Prefix :: * ; # [rustfmt :: skip] return match prefix { Some (Kilo | Kibi) => self . ui . size . unwrap_or_default () . unit_kilo () , Some (Mega | Mebi) => self . ui . size . unwrap_or_default () . unit_mega () , Some (Giga | Gibi) => self . ui . size . unwrap_or_default () . unit_giga () , Some (_) => self . ui . size . unwrap_or_default () . unit_huge () , None => self . ui . size . unwrap_or_default () . unit_byte () , } ; } # [rustfmt :: skip] fn no_size (& self) -> Style { self . ui . punctuation () } # [rustfmt :: skip] fn major (& self) -> Style { self . ui . size . unwrap_or_default () . major () } # [rustfmt :: skip] fn comma (& self) -> Style { self . ui . punctuation () } # [rustfmt :: skip] fn minor (& self) -> Style { self . ui . size . unwrap_or_default () . minor () } }
};
}
