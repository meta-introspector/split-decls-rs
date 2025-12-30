// Generated macro for impl_6 (impl)
macro_rules! Depcrate_proptest_implsimpl_6 {
() => {
// Module: crate::proptest_impls
// Provides: {"impl_6"}
// Dependencies: {}
# [doc = " The [`Arbitrary`] impl for `Box<Utf8Path>` returns a path with between 0 and 8 components,"] # [doc = " joined by the [`MAIN_SEPARATOR`](std::path::MAIN_SEPARATOR) for the platform. (Each component is"] # [doc = " randomly generated, and may itself contain one or more separators.)"] # [doc = ""] # [doc = " On Unix, this generates an absolute path half of the time and a relative path the other half."] # [doc = ""] # [doc = " On Windows, this implementation doesn't currently generate"] # [doc = " [`Utf8PrefixComponent`](crate::Utf8PrefixComponent) instances, though in the future it might."] # [cfg (feature = "proptest1")] impl Arbitrary for Box < Utf8Path > { type Parameters = < Utf8PathBuf as Arbitrary > :: Parameters ; type Strategy = MapInto < StrategyFor < Utf8PathBuf > , Self > ; fn arbitrary_with (args : Self :: Parameters) -> Self :: Strategy { any_with :: < Utf8PathBuf > (args) . prop_map_into () } }
};
}
