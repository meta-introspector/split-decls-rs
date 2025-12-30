// Generated macro for impl_590 (impl)
macro_rules! Depcrate_errorimpl_590 {
() => {
// Module: crate::error
// Provides: {"impl_590"}
// Dependencies: {}
impl Message { fn format (& mut self , cmd : & Command , usage : Option < StyledStr >) { match self { Message :: Raw (s) => { let mut message = String :: new () ; std :: mem :: swap (s , & mut message) ; let styled = format :: format_error_message (& message , cmd . get_styles () , Some (cmd) , usage . as_ref () ,) ; * self = Self :: Formatted (styled) ; } Message :: Formatted (_) => { } } } fn formatted (& self , styles : & Styles) -> Cow < '_ , StyledStr > { match self { Message :: Raw (s) => { let styled = format :: format_error_message (s , styles , None , None) ; Cow :: Owned (styled) } Message :: Formatted (s) => Cow :: Borrowed (s) , } } }
};
}
