// Generated macro for impl_220 (impl)
macro_rules! Depcrate_sys_timeimpl_220 {
() => {
// Module: crate::sys::time
// Provides: {"impl_220"}
// Dependencies: {}
impl fmt :: Display for TimeSpec { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let (abs , sign) = if self . tv_sec () < 0 { (- * self , "-") } else { (* self , "") } ; let sec = abs . tv_sec () ; write ! (f , "{sign}") ? ; if abs . tv_nsec () == 0 { if sec == 1 { write ! (f , "1 second") ? ; } else { write ! (f , "{sec} seconds") ? ; } } else if abs . tv_nsec () % 1_000_000 == 0 { write ! (f , "{sec}.{:03} seconds" , abs . tv_nsec () / 1_000_000) ? ; } else if abs . tv_nsec () % 1_000 == 0 { write ! (f , "{sec}.{:06} seconds" , abs . tv_nsec () / 1_000) ? ; } else { write ! (f , "{sec}.{:09} seconds" , abs . tv_nsec ()) ? ; } Ok (()) } }
};
}
