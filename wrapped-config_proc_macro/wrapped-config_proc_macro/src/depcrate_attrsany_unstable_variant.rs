// Generated macro for any_unstable_variant (function)
macro_rules! Depcrate_attrsany_unstable_variant {
() => {
// Module: crate::attrs
// Provides: {"any_unstable_variant"}
// Dependencies: {}
# [doc = " Returns `true` if the there is at least one `unstable` attribute in the given slice."] pub fn any_unstable_variant (attrs : & [syn :: Attribute]) -> bool { attrs . iter () . any (is_unstable_variant) }
};
}
