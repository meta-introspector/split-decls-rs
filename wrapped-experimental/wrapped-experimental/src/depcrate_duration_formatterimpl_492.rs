// Generated macro for impl_492 (impl)
macro_rules! Depcrate_duration_formatterimpl_492 {
() => {
// Module: crate::duration::formatter
// Provides: {"impl_492"}
// Dependencies: {}
impl core :: ops :: Index < Unit > for DurationUnitFormatter { type Output = UnitsFormatter ; fn index (& self , index : Unit) -> & Self :: Output { match index { Unit :: Year => & self . year , Unit :: Month => & self . month , Unit :: Week => & self . week , Unit :: Day => & self . day , Unit :: Hour => & self . hour , Unit :: Minute => & self . minute , Unit :: Second => & self . second , Unit :: Millisecond => & self . millisecond , Unit :: Microsecond => & self . microsecond , Unit :: Nanosecond => & self . nanosecond , } } }
};
}
