// Generated macro for impl_55 (impl)
macro_rules! Depcrateimpl_55 {
() => {
// Module: crate
// Provides: {"impl_55"}
// Dependencies: {}
impl < B : BitBlock > Deref for MutBorrowedBit < '_ , B > { type Target = bool ; fn deref (& self) -> & Self :: Target { & self . new_value } }
};
}
