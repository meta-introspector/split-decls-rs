// Generated macro for impl_1005 (impl)
macro_rules! Depcrate_themeimpl_1005 {
() => {
// Module: crate::theme
// Provides: {"impl_1005"}
// Dependencies: {}
# [rustfmt :: skip] impl render :: FiletypeColours for Theme { fn normal (& self) -> Style { self . ui . filekinds . unwrap_or_default () . normal () } fn directory (& self) -> Style { self . ui . filekinds . unwrap_or_default () . directory () } fn pipe (& self) -> Style { self . ui . filekinds . unwrap_or_default () . pipe () } fn symlink (& self) -> Style { self . ui . filekinds . unwrap_or_default () . symlink () } fn block_device (& self) -> Style { self . ui . filekinds . unwrap_or_default () . block_device () } fn char_device (& self) -> Style { self . ui . filekinds . unwrap_or_default () . char_device () } fn socket (& self) -> Style { self . ui . filekinds . unwrap_or_default () . socket () } fn special (& self) -> Style { self . ui . filekinds . unwrap_or_default () . special () } }
};
}
