// Generated macro for impl_123 (impl)
macro_rules! Depcrate_parseimpl_123 {
() => {
// Module: crate::parse
// Provides: {"impl_123"}
// Dependencies: {}
impl < 'a > Index < LinkIndex > for Allocations < 'a > { type Output = (LinkType , CowStr < 'a > , CowStr < 'a > , CowStr < 'a >) ; fn index (& self , ix : LinkIndex) -> & Self :: Output { self . links . index (ix . 0) } }
};
}
