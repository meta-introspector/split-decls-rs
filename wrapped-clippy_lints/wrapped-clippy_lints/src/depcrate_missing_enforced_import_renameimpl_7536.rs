// Generated macro for impl_7536 (impl)
macro_rules! Depcrate_missing_enforced_import_renameimpl_7536 {
() => {
// Module: crate::missing_enforced_import_rename
// Provides: {"impl_7536"}
// Dependencies: {}
impl LateLintPass < '_ > for ImportRename { fn check_item (& mut self , cx : & LateContext < '_ > , item : & Item < '_ >) { if let ItemKind :: Use (path , UseKind :: Single (_)) = & item . kind { for res in path . res . present_items () { if let Res :: Def (_ , id) = res && let Some (name) = self . renames . get (& id) && let span_without_semi = cx . sess () . source_map () . span_until_char (item . span , ';') && let Some (snip) = span_without_semi . get_source_text (cx) && let Some (import) = match snip . split_once (" as ") { None => Some (snip . as_str ()) , Some ((import , rename)) => { if rename . trim () == name . as_str () { None } else { Some (import . trim ()) } } , } { span_lint_and_sugg (cx , MISSING_ENFORCED_IMPORT_RENAMES , span_without_semi , "this import should be renamed" , "try" , format ! ("{import} as {name}" ,) , Applicability :: MachineApplicable ,) ; } } } } }
};
}
