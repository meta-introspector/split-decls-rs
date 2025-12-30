// Generated macro for impl_29 (impl)
macro_rules! Depcrate_decoderimpl_29 {
() => {
// Module: crate::decoder
// Provides: {"impl_29"}
// Dependencies: {}
impl < 'i > Iterator for LineReader < 'i > { type Item = Result < Line < 'i > , Error > ; fn next (& mut self) -> Option < Result < Line < 'i > , Error > > { if let Some (line_width) = self . line_width { let rest = match self . remaining . get (line_width ..) { None | Some ([]) => { if self . remaining . is_empty () { return None ; } else { let line = Line :: new (self . remaining) . trim_end () ; self . remaining = & [] ; return Some (Ok (line)) ; } } Some ([CHAR_CR , CHAR_LF , rest @ ..]) => rest , Some ([CHAR_CR , rest @ ..]) => rest , Some ([CHAR_LF , rest @ ..]) => rest , _ => { return Some (Err (Error :: InvalidEncoding)) ; } } ; let line = Line :: new (& self . remaining [.. line_width]) ; self . remaining = rest ; Some (Ok (line)) } else if ! self . remaining . is_empty () { let line = Line :: new (self . remaining) . trim_end () ; self . remaining = b"" ; if line . is_empty () { None } else { Some (Ok (line)) } } else { None } } }
};
}
