// Generated macro for impl_30 (impl)
macro_rules! Depcrate_implsimpl_30 {
() => {
// Module: crate::impls
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a > TryFrom < std :: borrow :: Cow < 'a , BStr > > for Url { type Error = parse :: Error ; fn try_from (value : std :: borrow :: Cow < 'a , BStr >) -> Result < Self , Self :: Error > { Self :: try_from (& * value) } }
};
}
