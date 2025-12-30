// Generated macro for walk_enum_def (function)
macro_rules! Depcrate_intravisitwalk_enum_def {
() => {
// Module: crate::intravisit
// Provides: {"walk_enum_def"}
// Dependencies: {}
pub fn walk_enum_def < 'v , V : Visitor < 'v > > (visitor : & mut V , enum_definition : & 'v EnumDef < 'v > ,) -> V :: Result { let EnumDef { variants } = enum_definition ; walk_list ! (visitor , visit_variant , * variants) ; V :: Result :: output () }
};
}
