// Generated macro for impl_1013 (impl)
macro_rules! Depcrate_themeimpl_1013 {
() => {
// Module: crate::theme
// Provides: {"impl_1013"}
// Dependencies: {}
# [rustfmt :: skip] impl FileNameColours for Theme { fn symlink_path (& self) -> Style { self . ui . symlink_path () } fn normal_arrow (& self) -> Style { self . ui . punctuation () } fn broken_symlink (& self) -> Style { self . ui . broken_symlink () } fn broken_filename (& self) -> Style { apply_overlay (self . ui . broken_symlink () , self . ui . broken_path_overlay ()) } fn control_char (& self) -> Style { self . ui . control_char () } fn broken_control_char (& self) -> Style { apply_overlay (self . ui . control_char () , self . ui . broken_path_overlay ()) } fn executable_file (& self) -> Style { self . ui . filekinds . unwrap_or_default () . executable () } fn mount_point (& self) -> Style { self . ui . filekinds . unwrap_or_default () . mount_point () } fn colour_file (& self , file : & File < '_ >) -> Style { self . exts . get_style (file , self) . unwrap_or (self . ui . filekinds . unwrap_or_default () . normal ()) } fn style_override (& self , file : & File < '_ >) -> Option < FileNameStyle > { if let Some (ref name_overrides) = self . ui . filenames { if let Some (file_override) = name_overrides . get (& file . name) { return Some (* file_override) ; } } if let Some (ref ext_overrides) = self . ui . extensions { if let Some (ext) = file . ext . clone () { if let Some (file_override) = ext_overrides . get (& ext) { return Some (* file_override) ; } } } None } }
};
}
