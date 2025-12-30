// Generated macro for impl_37 (impl)
macro_rules! Depcrate_interfaceimpl_37 {
() => {
// Module: crate::interface
// Provides: {"impl_37"}
// Dependencies: {}
impl TypeOwnershipStyle { # [doc = " Preserves this mode except for `OnlyTopBorrowed` where it switches it to"] # [doc = " `Owned`."] fn next (& self) -> TypeOwnershipStyle { match self { TypeOwnershipStyle :: Owned => TypeOwnershipStyle :: Owned , TypeOwnershipStyle :: Borrowed => TypeOwnershipStyle :: Borrowed , TypeOwnershipStyle :: OnlyTopBorrowed => TypeOwnershipStyle :: Owned , } } }
};
}
