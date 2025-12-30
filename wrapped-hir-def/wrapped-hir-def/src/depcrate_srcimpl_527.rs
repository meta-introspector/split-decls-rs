// Generated macro for impl_527 (impl)
macro_rules! Depcrate_srcimpl_527 {
() => {
// Module: crate::src
// Provides: {"impl_527"}
// Dependencies: {}
impl HasChildSource < la_arena :: Idx < ast :: UseTree > > for UseId { type Value = ast :: UseTree ; fn child_source (& self , db : & dyn DefDatabase ,) -> InFile < ArenaMap < la_arena :: Idx < ast :: UseTree > , Self :: Value > > { let loc = self . lookup (db) ; InFile :: new (loc . id . file_id , use_tree_source_map (db , loc . id) . into_iter () . collect ()) } }
};
}
