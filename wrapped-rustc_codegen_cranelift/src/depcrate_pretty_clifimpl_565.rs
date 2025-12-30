// Generated macro for impl_565 (impl)
macro_rules! Depcrate_pretty_clifimpl_565 {
() => {
// Module: crate::pretty_clif
// Provides: {"impl_565"}
// Dependencies: {}
impl CommentWriter { pub (crate) fn enabled (& self) -> bool { self . enabled } pub (crate) fn add_global_comment < S : Into < String > > (& mut self , comment : S) { debug_assert ! (self . enabled) ; self . global_comments . push (comment . into ()) ; } pub (crate) fn add_comment < S : Into < String > + AsRef < str > , E : Into < AnyEntity > > (& mut self , entity : E , comment : S ,) { debug_assert ! (self . enabled) ; use std :: collections :: hash_map :: Entry ; match self . entity_comments . entry (entity . into ()) { Entry :: Occupied (mut occ) => { occ . get_mut () . push ('\n') ; occ . get_mut () . push_str (comment . as_ref ()) ; } Entry :: Vacant (vac) => { vac . insert (comment . into ()) ; } } } pub (crate) fn add_post_comment < S : Into < String > + AsRef < str > > (& mut self , entity : Inst , comment : S ,) { debug_assert ! (self . enabled) ; use std :: collections :: hash_map :: Entry ; match self . inst_post_comments . entry (entity) { Entry :: Occupied (mut occ) => { occ . get_mut () . push ('\n') ; occ . get_mut () . push_str (comment . as_ref ()) ; } Entry :: Vacant (vac) => { vac . insert (comment . into ()) ; } } } }
};
}
