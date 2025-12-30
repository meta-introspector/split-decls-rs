// Generated macro for impl_132 (impl)
macro_rules! Depcrate_value_integerimpl_132 {
() => {
// Module: crate::value::integer
// Provides: {"impl_132"}
// Dependencies: {}
impl TryFrom < i128 > for Integer { type Error = core :: num :: TryFromIntError ; # [inline] fn try_from (value : i128) -> Result < Self , Self :: Error > { u64 :: try_from (match value . is_negative () { false => value , true => value ^ ! 0 , }) ? ; Ok (Integer (value)) } }
};
}
