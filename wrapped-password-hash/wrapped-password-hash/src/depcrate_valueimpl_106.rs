// Generated macro for impl_106 (impl)
macro_rules! Depcrate_valueimpl_106 {
() => {
// Module: crate::value
// Provides: {"impl_106"}
// Dependencies: {}
impl < 'a > TryFrom < Value < 'a > > for Decimal { type Error = Error ; fn try_from (value : Value < 'a >) -> Result < Decimal > { Decimal :: try_from (& value) } }
};
}
