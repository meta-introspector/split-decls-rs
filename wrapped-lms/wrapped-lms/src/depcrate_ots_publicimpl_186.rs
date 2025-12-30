// Generated macro for impl_186 (impl)
macro_rules! Depcrate_ots_publicimpl_186 {
() => {
// Module: crate::ots::public
// Provides: {"impl_186"}
// Dependencies: {}
impl < Mode : LmsOtsMode > Clone for VerifyingKey < Mode > { fn clone (& self) -> Self { Self { q : self . q , id : self . id , k : self . k . clone () , } } }
};
}
