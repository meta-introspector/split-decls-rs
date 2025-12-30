// Generated macro for to_roff (function)
macro_rules! Depcrateto_roff {
() => {
// Module: crate
// Provides: {"to_roff"}
// Dependencies: {}
# [doc = " Generate a [`Roff`] from ANSI escape codes"] # [doc = ""] # [doc = " ```rust"] # [doc = " let text = \"\\u{1b}[44;31mtest\\u{1b}[0m\";"] # [doc = ""] # [doc = " let roff_doc = anstyle_roff::to_roff(text);"] # [doc = " let expected = r#\".gcolor red"] # [doc = " .fcolor blue"] # [doc = " test"] # [doc = " \"#;"] # [doc = ""] # [doc = " assert_eq!(roff_doc.to_roff(), expected);"] # [doc = " ```"] pub fn to_roff (styled_text : & str) -> Roff { let mut doc = Roff :: new () ; let mut previous_fg_color = None ; let mut previous_bg_color = None ; for styled in styled_str :: styled_stream (styled_text) { if previous_fg_color != styled . style . get_fg_color () { add_color_to_roff (& mut doc , control_requests :: FOREGROUND , & styled . style . get_fg_color () ,) ; previous_fg_color = styled . style . get_fg_color () ; } if previous_bg_color != styled . style . get_bg_color () { add_color_to_roff (& mut doc , control_requests :: BACKGROUND , & styled . style . get_bg_color () ,) ; previous_bg_color = styled . style . get_bg_color () ; } set_effects_and_text (& styled , & mut doc) ; } doc }
};
}
