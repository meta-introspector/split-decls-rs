// Generated macro for impl_767 (impl)
macro_rules! Depcrate_spanimpl_767 {
() => {
// Module: crate::span
// Provides: {"impl_767"}
// Dependencies: {}
# [cfg (test)] impl quickcheck :: Arbitrary for Span { fn arbitrary (g : & mut quickcheck :: Gen) -> Span { type Nanos = ri64 < - 631_107_417_600_000_000 , 631_107_417_600_000_000 > ; let nanos = Nanos :: arbitrary (g) . get () ; let relative = SpanRelativeTo :: from (DateTime :: constant (0 , 1 , 1 , 0 , 0 , 0 , 0)) ; let round = SpanRound :: new () . largest (Unit :: arbitrary (g)) . relative (relative) ; Span :: new () . nanoseconds (nanos) . round (round) . unwrap () } fn shrink (& self) -> alloc :: boxed :: Box < dyn Iterator < Item = Self > > { alloc :: boxed :: Box :: new (((self . get_years_ranged () , self . get_months_ranged () , self . get_weeks_ranged () , self . get_days_ranged () ,) , (self . get_hours_ranged () , self . get_minutes_ranged () , self . get_seconds_ranged () , self . get_milliseconds_ranged () ,) , (self . get_microseconds_ranged () , self . get_nanoseconds_ranged () ,) ,) . shrink () . filter_map (| ((years , months , weeks , days) , (hours , minutes , seconds , milliseconds) , (microseconds , nanoseconds) ,) | { let span = Span :: new () . years_ranged (years) . months_ranged (months) . weeks_ranged (weeks) . days_ranged (days) . hours_ranged (hours) . minutes_ranged (minutes) . seconds_ranged (seconds) . milliseconds_ranged (milliseconds) . microseconds_ranged (microseconds) . nanoseconds_ranged (nanoseconds) ; Some (span) } ,) ,) } }
};
}
