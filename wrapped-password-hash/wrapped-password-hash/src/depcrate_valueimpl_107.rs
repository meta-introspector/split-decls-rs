// Generated macro for impl_107 (impl)
macro_rules! Depcrate_valueimpl_107 {
() => {
// Module: crate::value
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'a > TryFrom < & Value < 'a > > for Decimal { type Error = Error ; fn try_from (value : & Value < 'a >) -> Result < Decimal > { value . decimal () } }
};
}
