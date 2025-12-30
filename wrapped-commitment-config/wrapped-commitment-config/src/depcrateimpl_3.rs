// Generated macro for impl_3 (impl)
macro_rules! Depcrateimpl_3 {
() => {
// Module: crate
// Provides: {"impl_3"}
// Dependencies: {}
impl CommitmentConfig { pub const fn finalized () -> Self { Self { commitment : CommitmentLevel :: Finalized , } } pub const fn confirmed () -> Self { Self { commitment : CommitmentLevel :: Confirmed , } } pub const fn processed () -> Self { Self { commitment : CommitmentLevel :: Processed , } } pub fn ok (self) -> Option < Self > { if self == Self :: default () { None } else { Some (self) } } pub fn is_finalized (& self) -> bool { self . commitment == CommitmentLevel :: Finalized } pub fn is_confirmed (& self) -> bool { self . commitment == CommitmentLevel :: Confirmed } pub fn is_processed (& self) -> bool { self . commitment == CommitmentLevel :: Processed } pub fn is_at_least_confirmed (& self) -> bool { self . is_confirmed () || self . is_finalized () } }
};
}
