// Generated macro for hir_display_with_store (function)
macro_rules! Depcrate_displayhir_display_with_store {
() => {
// Module: crate::display
// Provides: {"hir_display_with_store"}
// Dependencies: {}
pub fn hir_display_with_store < 'a , 'db , T : HirDisplayWithExpressionStore < 'db > + 'a > (value : T , store : & 'a ExpressionStore ,) -> impl HirDisplay < 'db > + 'a { ExpressionStoreAdapter (value , store) }
};
}
