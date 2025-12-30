// Generated macro for impl_36 (impl)
macro_rules! Depcrate_referencedimpl_36 {
() => {
// Module: crate::referenced
// Provides: {"impl_36"}
// Dependencies: {}
impl < T > OwnedToRef for Option < T > where T : OwnedToRef , { type Borrowed < 'a > = Option < T :: Borrowed < 'a > > where T : 'a ; fn owned_to_ref (& self) -> Self :: Borrowed < '_ > { self . as_ref () . map (| o | o . owned_to_ref ()) } }
};
}
