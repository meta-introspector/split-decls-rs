// Generated macro for impl_45 (impl)
macro_rules! Depcrate_shared_posiximpl_45 {
() => {
// Module: crate::shared::posix
// Provides: {"impl_45"}
// Dependencies: {}
impl core :: fmt :: Display for PosixTime { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { if self . second . is_negative () { write ! (f , "-") ? ; } let second = self . second . unsigned_abs () ; let h = second / 3600 ; let m = (second / 60) % 60 ; let s = second % 60 ; write ! (f , "{h}") ? ; if m != 0 || s != 0 { write ! (f , ":{m:02}") ? ; if s != 0 { write ! (f , ":{s:02}") ? ; } } Ok (()) } }
};
}
