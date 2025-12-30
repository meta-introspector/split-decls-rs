// Generated macro for check (function)
macro_rules! Depcrate_attrs_unnecessary_clippy_cfgcheck {
() => {
// Module: crate::attrs::unnecessary_clippy_cfg
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & EarlyContext < '_ > , cfg_attr : & rustc_ast :: MetaItem , behind_cfg_attr : & rustc_ast :: MetaItem , attr : & Attribute ,) { if cfg_attr . has_name (sym :: clippy) && let Some (ident) = behind_cfg_attr . ident () && Level :: from_symbol (ident . name , | | Some (attr . id)) . is_some () && let Some (items) = behind_cfg_attr . meta_item_list () { let nb_items = items . len () ; let mut clippy_lints = Vec :: with_capacity (items . len ()) ; for item in items { if let Some (meta_item) = item . meta_item () && let [part1 , _] = meta_item . path . segments . as_slice () && part1 . ident . name == sym :: clippy { clippy_lints . push (item . span ()) ; } } if clippy_lints . is_empty () { return ; } if nb_items == clippy_lints . len () { if let Some (snippet) = behind_cfg_attr . span . get_source_text (cx) { span_lint_and_sugg (cx , UNNECESSARY_CLIPPY_CFG , attr . span , "no need to put clippy lints behind a `clippy` cfg" , "replace with" , format ! ("#{}[{}]" , if attr . style == AttrStyle :: Inner { "!" } else { "" } , snippet) , Applicability :: MachineApplicable ,) ; } } else { let snippet = clippy_lints . iter () . filter_map (| sp | sp . get_source_text (cx)) . join (",") ; span_lint_and_note (cx , UNNECESSARY_CLIPPY_CFG , clippy_lints , "no need to put clippy lints behind a `clippy` cfg" , None , format ! ("write instead: `#{}[{}({})]`" , if attr . style == AttrStyle :: Inner { "!" } else { "" } , ident . name , snippet) ,) ; } } }
};
}
