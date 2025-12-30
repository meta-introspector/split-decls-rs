// Generated macro for impl_659 (impl)
macro_rules! Depcrate_provider_pattern_reference_patternimpl_659 {
() => {
// Module: crate::provider::pattern::reference::pattern
// Provides: {"impl_659"}
// Dependencies: {}
impl Pattern { # [doc = " Convert a [`Pattern`] to a vector of pattern items."] # [doc = ""] # [doc = " The [`Pattern`] can be restored via the `From` impl."] pub fn into_items (self) -> Vec < PatternItem > { self . items } # [cfg (feature = "datagen")] pub (crate) fn items (& self) -> & [PatternItem] { & self . items } # [cfg (feature = "datagen")] pub (crate) fn items_mut (& mut self) -> & mut [PatternItem] { & mut self . items } # [cfg (any (feature = "serde" , test))] pub (crate) fn to_runtime_pattern (& self) -> runtime :: Pattern < 'static > { runtime :: Pattern :: from (self) } }
};
}
