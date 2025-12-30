// Generated macro for lang_item_from_well_known_trait (function)
macro_rules! Depcrate_chalk_dblang_item_from_well_known_trait {
() => {
// Module: crate::chalk_db
// Provides: {"lang_item_from_well_known_trait"}
// Dependencies: {}
fn lang_item_from_well_known_trait (trait_ : WellKnownTrait) -> LangItem { match trait_ { WellKnownTrait :: Clone => LangItem :: Clone , WellKnownTrait :: CoerceUnsized => LangItem :: CoerceUnsized , WellKnownTrait :: Copy => LangItem :: Copy , WellKnownTrait :: DiscriminantKind => LangItem :: DiscriminantKind , WellKnownTrait :: DispatchFromDyn => LangItem :: DispatchFromDyn , WellKnownTrait :: Drop => LangItem :: Drop , WellKnownTrait :: Fn => LangItem :: Fn , WellKnownTrait :: FnMut => LangItem :: FnMut , WellKnownTrait :: FnOnce => LangItem :: FnOnce , WellKnownTrait :: AsyncFn => LangItem :: AsyncFn , WellKnownTrait :: AsyncFnMut => LangItem :: AsyncFnMut , WellKnownTrait :: AsyncFnOnce => LangItem :: AsyncFnOnce , WellKnownTrait :: Coroutine => LangItem :: Coroutine , WellKnownTrait :: Sized => LangItem :: Sized , WellKnownTrait :: Tuple => LangItem :: Tuple , WellKnownTrait :: Unpin => LangItem :: Unpin , WellKnownTrait :: Unsize => LangItem :: Unsize , WellKnownTrait :: Pointee => LangItem :: PointeeTrait , WellKnownTrait :: FnPtr => LangItem :: FnPtrTrait , WellKnownTrait :: Future => LangItem :: Future , } }
};
}
