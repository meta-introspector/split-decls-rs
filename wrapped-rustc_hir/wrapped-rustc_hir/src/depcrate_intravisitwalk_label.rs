// Generated macro for walk_label (function)
macro_rules! Depcrate_intravisitwalk_label {
() => {
// Module: crate::intravisit
// Provides: {"walk_label"}
// Dependencies: {}
pub fn walk_label < 'v , V : Visitor < 'v > > (visitor : & mut V , label : & 'v Label) -> V :: Result { let Label { ident } = label ; visitor . visit_ident (* ident) }
};
}
