// Generated macro for impl_609 (impl)
macro_rules! Depcrate_shared_posiximpl_609 {
() => {
// Module: crate::shared::posix
// Provides: {"impl_609"}
// Dependencies: {}
impl core :: fmt :: Display for PosixDay { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { match * self { PosixDay :: JulianOne (n) => write ! (f , "J{n}") , PosixDay :: JulianZero (n) => write ! (f , "{n}") , PosixDay :: WeekdayOfMonth { month , week , weekday } => { write ! (f , "M{month}.{week}.{weekday}") } } } }
};
}
