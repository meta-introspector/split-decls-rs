// Generated macro for impl_36 (impl)
macro_rules! Depcrate_util_entityimpl_36 {
() => {
// Module: crate::util::entity
// Provides: {"impl_36"}
// Dependencies: {}
impl EntityTagRange { pub (crate) fn matches_strong (& self , entity : & EntityTag) -> bool { self . matches_if (entity , | a , b | a . strong_eq (b)) } pub (crate) fn matches_weak (& self , entity : & EntityTag) -> bool { self . matches_if (entity , | a , b | a . weak_eq (b)) } fn matches_if < F > (& self , entity : & EntityTag , func : F) -> bool where F : Fn (& EntityTag < & str > , & EntityTag) -> bool , { match * self { EntityTagRange :: Any => true , EntityTagRange :: Tags (ref tags) => tags . iter () . flat_map (EntityTag :: < & str > :: parse) . any (| tag | func (& tag , entity)) , } } }
};
}
