// Generated macro for impl_191 (impl)
macro_rules! Depcrate_refs_castimpl_191 {
() => {
// Module: crate::refs::cast
// Provides: {"impl_191"}
// Dependencies: {}
impl < 'local , 'from , To : Reference > AsRef < To :: Kind < 'local > > for Cast < 'local , 'from , To > { fn as_ref (& self) -> & To :: Kind < 'local > { & self . to } }
};
}
