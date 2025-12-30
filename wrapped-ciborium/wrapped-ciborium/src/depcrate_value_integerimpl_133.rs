// Generated macro for impl_133 (impl)
macro_rules! Depcrate_value_integerimpl_133 {
() => {
// Module: crate::value::integer
// Provides: {"impl_133"}
// Dependencies: {}
impl TryFrom < u128 > for Integer { type Error = core :: num :: TryFromIntError ; # [inline] fn try_from (value : u128) -> Result < Self , Self :: Error > { Ok (Self (u64 :: try_from (value) ? . into ())) } }
};
}
