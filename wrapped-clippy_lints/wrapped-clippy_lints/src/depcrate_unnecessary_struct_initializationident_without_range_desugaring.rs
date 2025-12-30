// Generated macro for ident_without_range_desugaring (function)
macro_rules! Depcrate_unnecessary_struct_initializationident_without_range_desugaring {
() => {
// Module: crate::unnecessary_struct_initialization
// Provides: {"ident_without_range_desugaring"}
// Dependencies: {}
fn ident_without_range_desugaring (ident : Ident) -> Ident { if ident . span . desugaring_kind () == Some (DesugaringKind :: RangeExpr) { Ident { span : ident . span . parent_callsite () . unwrap () , .. ident } } else { ident } }
};
}
