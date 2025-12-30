// Generated macro for impl_78 (impl)
macro_rules! Depcrate_commitimpl_78 {
() => {
// Module: crate::commit
// Provides: {"impl_78"}
// Dependencies: {}
# [doc = " Conversion"] impl CommitRef < '_ > { # [doc = " Copy all fields of this instance into a fully owned commit, consuming this instance."] pub fn into_owned (self) -> Result < Commit , crate :: decode :: Error > { self . try_into () } # [doc = " Copy all fields of this instance into a fully owned commit, internally cloning this instance."] pub fn to_owned (self) -> Result < Commit , crate :: decode :: Error > { self . try_into () } }
};
}
