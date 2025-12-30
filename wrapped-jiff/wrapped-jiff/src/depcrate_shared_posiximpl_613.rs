// Generated macro for impl_613 (impl)
macro_rules! Depcrate_shared_posiximpl_613 {
() => {
// Module: crate::shared::posix
// Provides: {"impl_613"}
// Dependencies: {}
impl core :: fmt :: Display for PosixOffset { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { if self . second > 0 { write ! (f , "-") ? ; } let second = self . second . unsigned_abs () ; let h = second / 3600 ; let m = (second / 60) % 60 ; let s = second % 60 ; write ! (f , "{h}") ? ; if m != 0 || s != 0 { write ! (f , ":{m:02}") ? ; if s != 0 { write ! (f , ":{s:02}") ? ; } } Ok (()) } }
};
}
