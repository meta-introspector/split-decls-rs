// Generated macro for impl_1372 (impl)
macro_rules! Depcrate_utils_cacheimpl_1372 {
() => {
// Module: crate::utils::cache
// Provides: {"impl_1372"}
// Dependencies: {}
# [cfg (test)] impl Cache { pub fn all < S : Ord + Step > (& mut self) -> Vec < (S , S :: Output) > { let cache = self . cache . get_mut () ; let type_id = TypeId :: of :: < S > () ; let mut v = cache . remove (& type_id) . map (| b | b . downcast :: < HashMap < S , S :: Output > > () . expect ("correct type")) . map (| m | m . into_iter () . collect :: < Vec < _ > > ()) . unwrap_or_default () ; v . sort_by_key (| (s , _) | s . clone ()) ; v } pub fn contains < S : Step > (& self) -> bool { self . cache . borrow () . contains_key (& TypeId :: of :: < S > ()) } # [cfg (test)] pub fn into_executed_steps (mut self) -> Vec < ExecutedStep > { mem :: take (& mut self . executed_steps . borrow_mut ()) } }
};
}
