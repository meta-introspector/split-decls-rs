// Generated macro for vertical (macro)
macro_rules! Depcrate_layoutvertical {
() => {
// Module: crate::layout
// Provides: {"vertical"}
// Dependencies: {}
# [doc = " Creates a vertical layout with specified constraints."] # [doc = ""] # [doc = " It accepts a series of constraints and applies them to create a vertical layout. The constraints"] # [doc = " can include fixed sizes, minimum and maximum sizes, percentages, and ratios."] # [doc = ""] # [doc = " See [`constraint!`]  or [`constraints!`] for more information."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " // Vertical layout with a fixed size and a percentage constraint"] # [doc = " use ratatui_macros::vertical;"] # [doc = " vertical![== 50, == 30%];"] # [doc = " ```"] # [macro_export] macro_rules ! vertical { ($ ($ constraint : tt) +) => { $ crate :: ratatui_core :: layout :: Layout :: vertical ($ crate :: constraints ! ($ ($ constraint) +)) } ; }
};
}
