// Generated macro for impl_1008 (impl)
macro_rules! Depcrate_themeimpl_1008 {
() => {
// Module: crate::theme
// Provides: {"impl_1008"}
// Dependencies: {}
# [rustfmt :: skip] # [cfg (unix)] impl render :: GroupColours for Theme { fn yours (& self) -> Style { self . ui . users . unwrap_or_default () . group_yours () } fn not_yours (& self) -> Style { self . ui . users . unwrap_or_default () . group_other () } fn root_group (& self) -> Style { self . ui . users . unwrap_or_default () . group_root () } fn no_group (& self) -> Style { self . ui . punctuation () } }
};
}
