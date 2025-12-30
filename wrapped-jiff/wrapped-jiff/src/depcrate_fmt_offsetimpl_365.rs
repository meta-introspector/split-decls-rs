// Generated macro for impl_365 (impl)
macro_rules! Depcrate_fmt_offsetimpl_365 {
() => {
// Module: crate::fmt::offset
// Provides: {"impl_365"}
// Dependencies: {}
impl core :: fmt :: Display for Numeric { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { if self . sign == C (- 1) { write ! (f , "-") ? ; } else { write ! (f , "+") ? ; } write ! (f , "{:02}" , self . hours) ? ; if let Some (minutes) = self . minutes { write ! (f , ":{:02}" , minutes) ? ; } if let Some (seconds) = self . seconds { write ! (f , ":{:02}" , seconds) ? ; } if let Some (nanos) = self . nanoseconds { static FMT : FractionalFormatter = FractionalFormatter :: new () ; write ! (f , ".{}" , FMT . format (i32 :: from (nanos) . unsigned_abs ()) . as_str ()) ? ; } Ok (()) } }
};
}
