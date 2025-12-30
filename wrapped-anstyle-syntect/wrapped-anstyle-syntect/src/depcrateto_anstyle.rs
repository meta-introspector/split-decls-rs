// Generated macro for to_anstyle (function)
macro_rules! Depcrateto_anstyle {
() => {
// Module: crate
// Provides: {"to_anstyle"}
// Dependencies: {}
# [doc = " Convert highlighting style to general style"] pub fn to_anstyle (style : syntect :: highlighting :: Style) -> anstyle :: Style { anstyle :: Style :: new () . fg_color (Some (to_anstyle_color (style . foreground))) . bg_color (Some (to_anstyle_color (style . background))) . effects (to_anstyle_effects (style . font_style)) }
};
}
