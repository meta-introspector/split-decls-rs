// Generated macro for from_attr_macro (function)
macro_rules! Depcrate_semicolon_if_nothing_returnedfrom_attr_macro {
() => {
// Module: crate::semicolon_if_nothing_returned
// Provides: {"from_attr_macro"}
// Dependencies: {}
fn from_attr_macro (span : Span) -> bool { matches ! (span . ctxt () . outer_expn_data () . kind , ExpnKind :: Macro (MacroKind :: Attr , _)) }
};
}
