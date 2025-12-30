// Generated macro for to_anstyle_effects (function)
macro_rules! Depcrateto_anstyle_effects {
() => {
// Module: crate
// Provides: {"to_anstyle_effects"}
// Dependencies: {}
# [doc = " Convert highlighting style to general effects"] pub fn to_anstyle_effects (style : syntect :: highlighting :: FontStyle) -> anstyle :: Effects { let mut effects = anstyle :: Effects :: new () ; if style . contains (syntect :: highlighting :: FontStyle :: BOLD) { effects |= anstyle :: Effects :: BOLD ; } if style . contains (syntect :: highlighting :: FontStyle :: ITALIC) { effects |= anstyle :: Effects :: ITALIC ; } if style . contains (syntect :: highlighting :: FontStyle :: UNDERLINE) { effects |= anstyle :: Effects :: UNDERLINE ; } effects }
};
}
