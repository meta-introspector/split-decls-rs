// Generated macro for impl_175 (impl)
macro_rules! Depcrateimpl_175 {
() => {
// Module: crate
// Provides: {"impl_175"}
// Dependencies: {}
impl EyreHandler for DefaultHandler { fn debug (& self , error : & (dyn StdError + 'static) , f : & mut core :: fmt :: Formatter < '_ > ,) -> core :: fmt :: Result { use core :: fmt :: Write as _ ; if f . alternate () { return core :: fmt :: Debug :: fmt (error , f) ; } write ! (f , "{error}") ? ; if let Some (cause) = error . source () { write ! (f , "\n\nCaused by:") ? ; let multiple = cause . source () . is_some () ; for (n , error) in crate :: chain :: Chain :: new (cause) . enumerate () { writeln ! (f) ? ; if multiple { write ! (indenter :: indented (f) . ind (n) , "{error}") ? ; } else { write ! (indenter :: indented (f) , "{error}") ? ; } } } # [cfg (all (track_caller , feature = "track-caller"))] { if let Some (location) = self . location { write ! (f , "\n\nLocation:\n") ? ; write ! (indenter :: indented (f) , "{location}") ? ; } } # [cfg (generic_member_access)] { use std :: backtrace :: BacktraceStatus ; let backtrace = self . backtrace . as_ref () . or_else (| | std :: error :: request_ref :: < Backtrace > (error)) . expect ("backtrace capture failed") ; if let BacktraceStatus :: Captured = backtrace . status () { write ! (f , "\n\nStack backtrace:\n{}" , backtrace) ? ; } } Result :: Ok (()) } # [cfg (track_caller)] fn track_caller (& mut self , location : & 'static std :: panic :: Location < 'static >) { self . location = Some (location) ; } }
};
}
