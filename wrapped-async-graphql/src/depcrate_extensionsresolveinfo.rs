// Generated macro for ResolveInfo (struct)
macro_rules! Depcrate_extensionsResolveInfo {
() => {
// Module: crate::extensions
// Provides: {"ResolveInfo"}
// Dependencies: {}
# [doc = " Parameters for `Extension::resolve_field_start`"] pub struct ResolveInfo < 'a > { # [doc = " Current path node, You can go through the entire path."] pub path_node : & 'a QueryPathNode < 'a > , # [doc = " Parent type"] pub parent_type : & 'a str , # [doc = " Current return type, is qualified name."] pub return_type : & 'a str , # [doc = " Current field name"] pub name : & 'a str , # [doc = " Current field alias"] pub alias : Option < & 'a str > , # [doc = " If `true` means the current field is for introspection."] pub is_for_introspection : bool , # [doc = " Current field"] pub field : & 'a Field , }
};
}
