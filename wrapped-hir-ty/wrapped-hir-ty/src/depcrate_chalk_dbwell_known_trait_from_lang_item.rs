// Generated macro for well_known_trait_from_lang_item (function)
macro_rules! Depcrate_chalk_dbwell_known_trait_from_lang_item {
() => {
// Module: crate::chalk_db
// Provides: {"well_known_trait_from_lang_item"}
// Dependencies: {}
fn well_known_trait_from_lang_item (item : LangItem) -> Option < WellKnownTrait > { Some (match item { LangItem :: Clone => WellKnownTrait :: Clone , LangItem :: CoerceUnsized => WellKnownTrait :: CoerceUnsized , LangItem :: Copy => WellKnownTrait :: Copy , LangItem :: DiscriminantKind => WellKnownTrait :: DiscriminantKind , LangItem :: DispatchFromDyn => WellKnownTrait :: DispatchFromDyn , LangItem :: Drop => WellKnownTrait :: Drop , LangItem :: Fn => WellKnownTrait :: Fn , LangItem :: FnMut => WellKnownTrait :: FnMut , LangItem :: FnOnce => WellKnownTrait :: FnOnce , LangItem :: AsyncFn => WellKnownTrait :: AsyncFn , LangItem :: AsyncFnMut => WellKnownTrait :: AsyncFnMut , LangItem :: AsyncFnOnce => WellKnownTrait :: AsyncFnOnce , LangItem :: Coroutine => WellKnownTrait :: Coroutine , LangItem :: Sized => WellKnownTrait :: Sized , LangItem :: Unpin => WellKnownTrait :: Unpin , LangItem :: Unsize => WellKnownTrait :: Unsize , LangItem :: Tuple => WellKnownTrait :: Tuple , LangItem :: PointeeTrait => WellKnownTrait :: Pointee , LangItem :: FnPtrTrait => WellKnownTrait :: FnPtr , LangItem :: Future => WellKnownTrait :: Future , _ => return None , }) }
};
}
