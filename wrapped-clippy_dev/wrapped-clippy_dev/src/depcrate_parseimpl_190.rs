// Generated macro for impl_190 (impl)
macro_rules! Depcrate_parseimpl_190 {
() => {
// Module: crate::parse
// Provides: {"impl_190"}
// Dependencies: {}
impl StrBuf { # [doc = " Creates a new buffer with the specified initial capacity."] pub fn with_capacity (cap : usize) -> Self { Self (String :: with_capacity (cap)) } # [doc = " Allocates the result of formatting the given value onto the arena."] pub fn alloc_display < 'cx > (& mut self , arena : & 'cx DroplessArena , value : impl Display) -> & 'cx str { self . 0 . clear () ; write ! (self . 0 , "{value}") . expect ("`Display` impl returned an error") ; arena . alloc_str (& self . 0) } # [doc = " Allocates the string onto the arena with all ascii characters converted to"] # [doc = " lowercase."] pub fn alloc_ascii_lower < 'cx > (& mut self , arena : & 'cx DroplessArena , s : & str) -> & 'cx str { self . 0 . clear () ; self . 0 . push_str (s) ; self . 0 . make_ascii_lowercase () ; arena . alloc_str (& self . 0) } # [doc = " Allocates the result of replacing all instances the pattern with the given string"] # [doc = " onto the arena."] pub fn alloc_replaced < 'cx > (& mut self , arena : & 'cx DroplessArena , s : & str , pat : impl Pattern , replacement : & str ,) -> & 'cx str { let mut parts = s . split (pat) ; let Some (first) = parts . next () else { return "" ; } ; self . 0 . clear () ; self . 0 . push_str (first) ; for part in parts { self . 0 . push_str (replacement) ; self . 0 . push_str (part) ; } if self . 0 . is_empty () { "" } else { arena . alloc_str (& self . 0) } } # [doc = " Performs an operation with the freshly cleared buffer."] pub fn with < T > (& mut self , f : impl FnOnce (& mut String) -> T) -> T { self . 0 . clear () ; f (& mut self . 0) } }
};
}
