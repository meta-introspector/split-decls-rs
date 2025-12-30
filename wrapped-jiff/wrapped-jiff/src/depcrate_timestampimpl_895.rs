// Generated macro for impl_895 (impl)
macro_rules! Depcrate_timestampimpl_895 {
() => {
// Module: crate::timestamp
// Provides: {"impl_895"}
// Dependencies: {}
# [cfg (test)] impl quickcheck :: Arbitrary for Timestamp { fn arbitrary (g : & mut quickcheck :: Gen) -> Timestamp { use quickcheck :: Arbitrary ; let seconds : UnixSeconds = Arbitrary :: arbitrary (g) ; let mut nanoseconds : FractionalNanosecond = Arbitrary :: arbitrary (g) ; if seconds == UnixSeconds :: MIN_SELF && nanoseconds < C (0) { nanoseconds = C (0) . rinto () ; } Timestamp :: new_ranged (seconds , nanoseconds) . unwrap_or_default () } fn shrink (& self) -> alloc :: boxed :: Box < dyn Iterator < Item = Self > > { let second = self . as_second_ranged () ; let nanosecond = self . subsec_nanosecond_ranged () ; alloc :: boxed :: Box :: new ((second , nanosecond) . shrink () . filter_map (| (second , nanosecond) | { if second == UnixSeconds :: MIN_SELF && nanosecond > C (0) { None } else { Timestamp :: new_ranged (second , nanosecond) . ok () } } ,)) } }
};
}
