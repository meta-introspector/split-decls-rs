// Generated macro for walk_mod (function)
macro_rules! Depcrate_intravisitwalk_mod {
() => {
// Module: crate::intravisit
// Provides: {"walk_mod"}
// Dependencies: {}
pub fn walk_mod < 'v , V : Visitor < 'v > > (visitor : & mut V , module : & 'v Mod < 'v >) -> V :: Result { let Mod { spans : _ , item_ids } = module ; walk_list ! (visitor , visit_nested_item , item_ids . iter () . copied ()) ; V :: Result :: output () }
};
}
