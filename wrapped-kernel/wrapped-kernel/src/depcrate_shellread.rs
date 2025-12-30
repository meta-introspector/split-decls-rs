// Generated macro for read (function)
macro_rules! Depcrate_shellread {
() => {
// Module: crate::shell
// Provides: {"read"}
// Dependencies: {}
fn read () -> Option < u8 > { let mut buf = [0 ; 1] ; let len = crate :: console :: CONSOLE . lock () . read (& mut buf) . ok () ? ; if len > 0 { Some (buf [0]) } else { None } }
};
}
