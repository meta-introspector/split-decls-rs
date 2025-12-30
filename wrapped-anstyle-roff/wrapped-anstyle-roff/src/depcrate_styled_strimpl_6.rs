// Generated macro for impl_6 (impl)
macro_rules! Depcrate_styled_strimpl_6 {
() => {
// Module: crate::styled_str
// Provides: {"impl_6"}
// Dependencies: {}
impl < 'text > From < CategorisedSlice < 'text > > for StyledStr < 'text > { fn from (category : CategorisedSlice < 'text >) -> Self { let mut style = Style :: new () ; style = style . fg_color (cansi_to_anstyle_color (category . fg)) . bg_color (cansi_to_anstyle_color (category . bg)) ; let effects = create_effects (& category) ; style = style . effects (effects) ; Self { text : category . text , style , } } }
};
}
