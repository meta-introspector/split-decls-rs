// Generated macro for impl_135 (impl)
macro_rules! Depcrate_value_integerimpl_135 {
() => {
// Module: crate::value::integer
// Provides: {"impl_135"}
// Dependencies: {}
impl TryFrom < Integer > for u128 { type Error = core :: num :: TryFromIntError ; # [inline] fn try_from (value : Integer) -> Result < Self , Self :: Error > { u128 :: try_from (value . 0) } }
};
}
