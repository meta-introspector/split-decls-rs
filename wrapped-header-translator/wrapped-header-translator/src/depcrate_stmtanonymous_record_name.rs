// Generated macro for anonymous_record_name (function)
macro_rules! Depcrate_stmtanonymous_record_name {
() => {
// Module: crate::stmt
// Provides: {"anonymous_record_name"}
// Dependencies: {}
pub (crate) fn anonymous_record_name (entity : & Entity < '_ > , context : & Context < '_ >) -> Option < String > { let parent = entity . get_semantic_parent () ? ; if ! matches ! (parent . get_kind () , EntityKind :: StructDecl | EntityKind :: UnionDecl) { return None ; } let parent_id = ItemIdentifier :: new_optional (& parent , context) . map_name (| name | name . or_else (| | anonymous_record_name (& parent , context))) . to_option () ? ; let mut just_found_record = false ; let mut field_name = None ; immediate_children (& parent , | searched , _span | match searched . get_kind () { EntityKind :: FieldDecl => { if just_found_record { field_name = Some (searched . get_name () . expect ("field name")) ; just_found_record = false ; } } EntityKind :: UnionDecl | EntityKind :: StructDecl => { if searched == * entity { just_found_record = true ; } } _ => { } }) ; let field_name = field_name ? ; Some (format ! ("{}_{}" , parent_id . name , field_name)) }
};
}
