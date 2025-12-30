// Generated macro for impl_194 (impl)
macro_rules! Depcrate_civil_timeimpl_194 {
() => {
// Module: crate::civil::time
// Provides: {"impl_194"}
// Dependencies: {}
# [cfg (test)] impl quickcheck :: Arbitrary for Time { fn arbitrary (g : & mut quickcheck :: Gen) -> Time { let hour = Hour :: arbitrary (g) ; let minute = Minute :: arbitrary (g) ; let second = Second :: arbitrary (g) ; let subsec_nanosecond = SubsecNanosecond :: arbitrary (g) ; Time :: new_ranged (hour , minute , second , subsec_nanosecond) } fn shrink (& self) -> alloc :: boxed :: Box < dyn Iterator < Item = Time > > { alloc :: boxed :: Box :: new ((self . hour_ranged () , self . minute_ranged () , self . second_ranged () , self . subsec_nanosecond_ranged () ,) . shrink () . map (| (hour , minute , second , subsec_nanosecond) | { Time :: new_ranged (hour , minute , second , subsec_nanosecond ,) } ,) ,) } }
};
}
