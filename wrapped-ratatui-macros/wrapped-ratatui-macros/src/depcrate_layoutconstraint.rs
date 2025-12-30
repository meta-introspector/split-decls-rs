// Generated macro for constraint (macro)
macro_rules! Depcrate_layoutconstraint {
() => {
// Module: crate::layout
// Provides: {"constraint"}
// Dependencies: {}
# [doc = " Creates a single constraint."] # [doc = ""] # [doc = " If creating an array of constraints, you probably want to use"] # [doc = " [`constraints!`] instead."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use ratatui_core::layout::Constraint;"] # [doc = " use ratatui_macros::constraint;"] # [doc = " assert_eq!(constraint!(>= 3 + 4), Constraint::Min(7));"] # [doc = " assert_eq!(constraint!(<= 3 + 4), Constraint::Max(7));"] # [doc = " assert_eq!(constraint!(== 1 / 3), Constraint::Ratio(1, 3));"] # [doc = " assert_eq!(constraint!(== 3), Constraint::Length(3));"] # [doc = " assert_eq!(constraint!(== 10 %), Constraint::Percentage(10));"] # [doc = " assert_eq!(constraint!(*= 1), Constraint::Fill(1));"] # [doc = " ```"] # [doc = ""] # [doc = " [`constraints!`]: crate::constraints"] # [macro_export] macro_rules ! constraint { (== $ token : tt %) => { $ crate :: ratatui_core :: layout :: Constraint :: Percentage ($ token) } ; (>= $ expr : expr) => { $ crate :: ratatui_core :: layout :: Constraint :: Min ($ expr) } ; (<= $ expr : expr) => { $ crate :: ratatui_core :: layout :: Constraint :: Max ($ expr) } ; (== $ num : tt / $ denom : tt) => { $ crate :: ratatui_core :: layout :: Constraint :: Ratio ($ num as u32 , $ denom as u32) } ; (== $ expr : expr) => { $ crate :: ratatui_core :: layout :: Constraint :: Length ($ expr) } ; (*= $ expr : expr) => { $ crate :: ratatui_core :: layout :: Constraint :: Fill ($ expr) } ; }
};
}
