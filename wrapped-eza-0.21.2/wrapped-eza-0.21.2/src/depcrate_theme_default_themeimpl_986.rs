// Generated macro for impl_986 (impl)
macro_rules! Depcrate_theme_default_themeimpl_986 {
() => {
// Module: crate::theme::default_theme
// Provides: {"impl_986"}
// Dependencies: {}
impl Size { pub fn colourful (scale : ColorScaleOptions) -> Self { if scale . size && scale . mode == ColorScaleMode :: Fixed { Self :: colourful_fixed () } else { Self :: colourful_gradient () } } fn colourful_fixed () -> Self { Self { major : Some (Green . bold ()) , minor : Some (Green . normal ()) , number_byte : Some (Green . bold ()) , number_kilo : Some (Green . bold ()) , number_mega : Some (Green . bold ()) , number_giga : Some (Green . bold ()) , number_huge : Some (Green . bold ()) , unit_byte : Some (Green . normal ()) , unit_kilo : Some (Green . normal ()) , unit_mega : Some (Green . normal ()) , unit_giga : Some (Green . normal ()) , unit_huge : Some (Green . normal ()) , } } fn colourful_gradient () -> Self { Self { major : Some (Green . bold ()) , minor : Some (Green . normal ()) , number_byte : Some (Green . normal ()) , number_kilo : Some (Green . bold ()) , number_mega : Some (Yellow . normal ()) , number_giga : Some (Red . normal ()) , number_huge : Some (Purple . normal ()) , unit_byte : Some (Green . normal ()) , unit_kilo : Some (Green . bold ()) , unit_mega : Some (Yellow . normal ()) , unit_giga : Some (Red . normal ()) , unit_huge : Some (Purple . normal ()) , } } }
};
}
