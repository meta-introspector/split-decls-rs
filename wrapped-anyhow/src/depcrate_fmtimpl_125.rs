// Generated macro for impl_125 (impl)
macro_rules! Depcrate_fmtimpl_125 {
() => {
// Module: crate::fmt
// Provides: {"impl_125"}
// Dependencies: {}
impl ErrorImpl { pub (crate) unsafe fn display (this : Ref < Self > , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}" , unsafe { Self :: error (this) }) ? ; if f . alternate () { let chain = unsafe { Self :: chain (this) } ; for cause in chain . skip (1) { write ! (f , ": {}" , cause) ? ; } } Ok (()) } pub (crate) unsafe fn debug (this : Ref < Self > , f : & mut fmt :: Formatter) -> fmt :: Result { let error = unsafe { Self :: error (this) } ; if f . alternate () { return Debug :: fmt (error , f) ; } write ! (f , "{}" , error) ? ; if let Some (cause) = error . source () { write ! (f , "\n\nCaused by:") ? ; let multiple = cause . source () . is_some () ; for (n , error) in Chain :: new (cause) . enumerate () { writeln ! (f) ? ; let mut indented = Indented { inner : f , number : if multiple { Some (n) } else { None } , started : false , } ; write ! (indented , "{}" , error) ? ; } } # [cfg (any (std_backtrace , feature = "backtrace"))] { use crate :: backtrace :: BacktraceStatus ; use alloc :: string :: ToString ; let backtrace = unsafe { Self :: backtrace (this) } ; if let BacktraceStatus :: Captured = backtrace . status () { let mut backtrace = backtrace . to_string () ; write ! (f , "\n\n") ? ; if backtrace . starts_with ("stack backtrace:") { backtrace . replace_range (0 .. 1 , "S") ; } else { writeln ! (f , "Stack backtrace:") ? ; } backtrace . truncate (backtrace . trim_end () . len ()) ; write ! (f , "{}" , backtrace) ? ; } } Ok (()) } }
};
}
