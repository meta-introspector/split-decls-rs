// Generated macro for impl_37 (impl)
macro_rules! Depcrate_referencedimpl_37 {
() => {
// Module: crate::referenced
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'a , T > RefToOwned < 'a > for Option < T > where T : RefToOwned < 'a > + 'a , T :: Owned : OwnedToRef , { type Owned = Option < T :: Owned > ; fn ref_to_owned (& self) -> Self :: Owned { self . as_ref () . map (| o | o . ref_to_owned ()) } }
};
}
