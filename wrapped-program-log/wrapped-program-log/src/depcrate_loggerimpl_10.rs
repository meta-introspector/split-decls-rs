// Generated macro for impl_10 (impl)
macro_rules! Depcrate_loggerimpl_10 {
() => {
// Module: crate::logger
// Provides: {"impl_10"}
// Dependencies: {}
impl < const BUFFER : usize > Logger < BUFFER > { # [doc = " Append a value to the logger."] # [inline (always)] pub fn append < T : Log > (& mut self , value : T) -> & mut Self { self . append_with_args (value , & []) ; self } # [doc = " Append a value to the logger with formatting arguments."] # [inline] pub fn append_with_args < T : Log > (& mut self , value : T , args : & [Argument]) -> & mut Self { if self . is_full () { if BUFFER > 0 { unsafe { let last = self . buffer . get_unchecked_mut (BUFFER - 1) ; last . write (TRUNCATED) ; } } } else { self . len += value . write_with_args (& mut self . buffer [self . len ..] , args) ; if self . len > BUFFER { self . len = BUFFER ; unsafe { let last = self . buffer . get_unchecked_mut (BUFFER - 1) ; last . write (TRUNCATED) ; } } } self } # [doc = " Log the message in the buffer."] # [inline (always)] pub fn log (& self) { log_message (self) ; } # [doc = " Clear the message buffer."] # [inline (always)] pub fn clear (& mut self) { self . len = 0 ; } # [doc = " Check whether the log buffer is at the maximum length or not."] # [inline (always)] pub fn is_full (& self) -> bool { self . len == BUFFER } # [doc = " Get the remaining space in the log buffer."] # [inline (always)] pub fn remaining (& self) -> usize { BUFFER - self . len } }
};
}
