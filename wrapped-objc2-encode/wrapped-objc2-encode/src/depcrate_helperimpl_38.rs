// Generated macro for impl_38 (impl)
macro_rules! Depcrate_helperimpl_38 {
() => {
// Module: crate::helper
// Provides: {"impl_38"}
// Dependencies: {}
impl NestingLevel { pub (crate) const fn new () -> Self { Self :: Top } pub (crate) const fn bitfield (self) -> Self { self } pub (crate) const fn indirection (self , kind : IndirectionKind) -> Self { match kind { IndirectionKind :: Atomic => Self :: Bottom , IndirectionKind :: Pointer => match self { Self :: Top => Self :: Within , Self :: Bottom | Self :: Within => Self :: Bottom , } , } } pub (crate) const fn array (self) -> Self { self } pub (crate) const fn container_include_fields (self) -> Option < Self > { match self { Self :: Top | Self :: Within => { Some (Self :: Within) } Self :: Bottom => None , } } }
};
}
