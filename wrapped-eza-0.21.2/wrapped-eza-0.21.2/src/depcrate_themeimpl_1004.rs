// Generated macro for impl_1004 (impl)
macro_rules! Depcrate_themeimpl_1004 {
() => {
// Module: crate::theme
// Provides: {"impl_1004"}
// Dependencies: {}
# [cfg (unix)] impl render :: BlocksColours for Theme { fn blocksize (& self , prefix : Option < number_prefix :: Prefix >) -> Style { use number_prefix :: Prefix :: * ; # [rustfmt :: skip] let style = match prefix { Some (Kilo | Kibi) => self . ui . size . unwrap_or_default () . number_kilo , Some (Mega | Mebi) => self . ui . size . unwrap_or_default () . number_mega , Some (Giga | Gibi) => self . ui . size . unwrap_or_default () . number_giga , Some (_) => self . ui . size . unwrap_or_default () . number_huge , None => self . ui . size . unwrap_or_default () . number_byte , } ; style . unwrap_or_default () } fn unit (& self , prefix : Option < number_prefix :: Prefix >) -> Style { use number_prefix :: Prefix :: * ; # [rustfmt :: skip] let style = match prefix { Some (Kilo | Kibi) => self . ui . size . unwrap_or_default () . unit_kilo , Some (Mega | Mebi) => self . ui . size . unwrap_or_default () . unit_mega , Some (Giga | Gibi) => self . ui . size . unwrap_or_default () . unit_giga , Some (_) => self . ui . size . unwrap_or_default () . unit_huge , None => self . ui . size . unwrap_or_default () . unit_byte , } ; style . unwrap_or_default () } fn no_blocksize (& self) -> Style { self . ui . punctuation . unwrap_or_default () } }
};
}
