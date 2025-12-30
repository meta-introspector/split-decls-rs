// Generated macro for impl_353 (impl)
macro_rules! Depcrate_inlay_hintsimpl_353 {
() => {
// Module: crate::inlay_hints
// Provides: {"impl_353"}
// Dependencies: {}
impl InlayHintsConfig < '_ > { fn lazy_text_edit (& self , finish : impl FnOnce () -> TextEdit) -> LazyProperty < TextEdit > { if self . fields_to_resolve . resolve_text_edits { LazyProperty :: Lazy } else { let edit = finish () ; never ! (edit . is_empty () , "inlay hint produced an empty text edit") ; LazyProperty :: Computed (edit) } } fn lazy_tooltip (& self , finish : impl FnOnce () -> InlayTooltip) -> LazyProperty < InlayTooltip > { if self . fields_to_resolve . resolve_hint_tooltip && self . fields_to_resolve . resolve_label_tooltip { LazyProperty :: Lazy } else { let tooltip = finish () ; never ! (match & tooltip { InlayTooltip :: String (s) => s , InlayTooltip :: Markdown (s) => s , } . is_empty () , "inlay hint produced an empty tooltip") ; LazyProperty :: Computed (tooltip) } } # [doc = " This always reports a resolvable location, so only use this when it is very likely for a"] # [doc = " location link to actually resolve but where computing `finish` would be costly."] fn lazy_location_opt (& self , finish : impl FnOnce () -> Option < FileRange > ,) -> Option < LazyProperty < FileRange > > { if self . fields_to_resolve . resolve_label_location { Some (LazyProperty :: Lazy) } else { finish () . map (LazyProperty :: Computed) } } }
};
}
