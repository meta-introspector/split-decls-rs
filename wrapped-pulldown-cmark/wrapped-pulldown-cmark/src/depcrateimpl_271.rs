// Generated macro for impl_271 (impl)
macro_rules! Depcrateimpl_271 {
() => {
// Module: crate
// Provides: {"impl_271"}
// Dependencies: {}
impl LinkType { # [doc = " Map the link type to an equivalent _Unknown link type."] fn to_unknown (self) -> Self { match self { LinkType :: Reference => LinkType :: ReferenceUnknown , LinkType :: Collapsed => LinkType :: CollapsedUnknown , LinkType :: Shortcut => LinkType :: ShortcutUnknown , _ => unreachable ! () , } } }
};
}
