// Generated macro for horizontal (macro)
macro_rules! Depcrate_layouthorizontal {
() => {
// Module: crate::layout
// Provides: {"horizontal"}
// Dependencies: {}
# [doc = " Creates a horizontal layout with specified constraints."] # [doc = ""] # [doc = " It takes a series of constraints and applies them to create a horizontal layout. The constraints"] # [doc = " can include fixed sizes, minimum and maximum sizes, percentages, and ratios."] # [doc = ""] # [doc = " See [`constraint!`]  or [`constraints!`] for more information."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " // Horizontal layout with a ratio constraint and a minimum size constraint"] # [doc = " use ratatui_macros::horizontal;"] # [doc = " horizontal![== 1/3, >= 100];"] # [doc = " ```"] # [macro_export] macro_rules ! horizontal { ($ ($ constraint : tt) +) => { $ crate :: ratatui_core :: layout :: Layout :: horizontal ($ crate :: constraints ! ($ ($ constraint) +)) } ; }
};
}
