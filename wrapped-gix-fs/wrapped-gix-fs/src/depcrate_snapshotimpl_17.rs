// Generated macro for impl_17 (impl)
macro_rules! Depcrate_snapshotimpl_17 {
() => {
// Module: crate::snapshot
// Provides: {"impl_17"}
// Dependencies: {}
impl < T : Clone + std :: fmt :: Debug > FileSnapshot < T > { # [doc = " Return the contained instance if nobody else is holding it, or clone it otherwise."] pub fn into_owned_or_cloned (self : OwnShared < Self >) -> T { match OwnShared :: try_unwrap (self) { Ok (this) => this . value , Err (this) => this . value . clone () , } } }
};
}
