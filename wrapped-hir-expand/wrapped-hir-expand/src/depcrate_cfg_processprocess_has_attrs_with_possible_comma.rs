// Generated macro for process_has_attrs_with_possible_comma (function)
macro_rules! Depcrate_cfg_processprocess_has_attrs_with_possible_comma {
() => {
// Module: crate::cfg_process
// Provides: {"process_has_attrs_with_possible_comma"}
// Dependencies: {}
fn process_has_attrs_with_possible_comma < I : HasAttrs > (db : & dyn ExpandDatabase , items : impl Iterator < Item = I > , krate : Crate , remove : & mut FxHashSet < SyntaxElement > ,) -> Option < () > { for item in items { let field_attrs = item . attrs () ; 'attrs : for attr in field_attrs { if let Some (enabled) = check_cfg (db , & attr , krate) { if enabled { debug ! ("censoring {:?}" , attr . syntax ()) ; remove . insert (attr . syntax () . clone () . into ()) ; } else { debug ! ("censoring {:?}" , item . syntax ()) ; remove . insert (item . syntax () . clone () . into ()) ; remove_possible_comma (& item , remove) ; break 'attrs ; } } if let Some (enabled) = check_cfg_attr (db , & attr , krate) { if enabled { debug ! ("Removing cfg_attr tokens {:?}" , attr) ; let meta = attr . meta () ? ; let removes_from_cfg_attr = remove_tokens_within_cfg_attr (meta) ? ; remove . extend (removes_from_cfg_attr) ; } else { debug ! ("censoring type cfg_attr {:?}" , item . syntax ()) ; remove . insert (attr . syntax () . clone () . into ()) ; } } } } Some (()) }
};
}
