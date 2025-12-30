// Generated macro for impl_27 (impl)
macro_rules! Depcrate_configimpl_27 {
() => {
// Module: crate::config
// Provides: {"impl_27"}
// Dependencies: {}
impl fmt :: Display for SourceSection < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let Self (frame , theme) = self ; let (lineno , filename) = match (frame . lineno , frame . filename . as_ref ()) { (Some (a) , Some (b)) => (a , b) , _ => return Ok (()) , } ; let file = match std :: fs :: File :: open (filename) { Ok (file) => file , Err (ref e) if e . kind () == std :: io :: ErrorKind :: NotFound => return Ok (()) , e @ Err (_) => e . unwrap () , } ; use std :: fmt :: Write ; use std :: io :: BufRead ; let reader = std :: io :: BufReader :: new (file) ; let start_line = lineno - 2 . min (lineno - 1) ; let surrounding_src = reader . lines () . skip (start_line as usize - 1) . take (5) ; let mut separated = f . header ("\n") ; let mut f = separated . in_progress () ; for (line , cur_line_no) in surrounding_src . zip (start_line ..) { let line = line . unwrap () ; if cur_line_no == lineno { write ! (& mut f , "{:>8} {} {}" , cur_line_no . style (theme . active_line) , ">" . style (theme . active_line) , line . style (theme . active_line) ,) ? ; } else { write ! (& mut f , "{cur_line_no:>8} │ {line}") ? ; } f = separated . ready () ; } Ok (()) } }
};
}
