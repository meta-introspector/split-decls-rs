// Generated macro for impl_416 (impl)
macro_rules! Depcrate_referenceimpl_416 {
() => {
// Module: crate::reference
// Provides: {"impl_416"}
// Dependencies: {}
# [doc = " Access"] impl < 'repo > Reference < 'repo > { # [doc = " Returns the attached id we point to, or `None` if this is a symbolic ref."] pub fn try_id (& self) -> Option < Id < 'repo > > { match self . inner . target { gix_ref :: Target :: Symbolic (_) => None , gix_ref :: Target :: Object (oid) => oid . to_owned () . attach (self . repo) . into () , } } # [doc = " Returns the attached id we point to, or panic if this is a symbolic ref."] pub fn id (& self) -> Id < 'repo > { self . try_id () . expect ("BUG: tries to obtain object id from symbolic target") } # [doc = " Return the target to which this reference points to."] pub fn target (& self) -> gix_ref :: TargetRef < '_ > { self . inner . target . to_ref () } # [doc = " Return the reference's full name."] pub fn name (& self) -> & gix_ref :: FullNameRef { self . inner . name . as_ref () } # [doc = " Turn this instances into a stand-alone reference."] pub fn detach (self) -> gix_ref :: Reference { self . inner } }
};
}
