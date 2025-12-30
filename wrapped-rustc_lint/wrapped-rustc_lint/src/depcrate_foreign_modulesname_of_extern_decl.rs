// Generated macro for name_of_extern_decl (function)
macro_rules! Depcrate_foreign_modulesname_of_extern_decl {
() => {
// Module: crate::foreign_modules
// Provides: {"name_of_extern_decl"}
// Dependencies: {}
# [doc = " Get the name of the symbol that's linked against for a given extern declaration. That is,"] # [doc = " the name specified in a #[link_name = ...] attribute if one was specified, else, just the"] # [doc = " symbol's name."] fn name_of_extern_decl (tcx : TyCtxt < '_ > , fi : hir :: OwnerId) -> SymbolName { if let Some ((overridden_link_name , overridden_link_name_span)) = tcx . codegen_fn_attrs (fi) . symbol_name . map (| overridden_link_name | { (overridden_link_name , find_attr ! (tcx . get_all_attrs (fi) , AttributeKind :: LinkName { span , .. } => * span) . unwrap () ,) }) { SymbolName :: Link (overridden_link_name , overridden_link_name_span) } else { SymbolName :: Normal (tcx . item_name (fi . to_def_id ())) } }
};
}
