// Generated macro for header_mut (function)
macro_rules! Depcrate_errorheader_mut {
() => {
// Module: crate::error
// Provides: {"header_mut"}
// Dependencies: {}
fn header_mut (p : MutPtr < '_ , ErrorImpl < () > >) -> & mut ErrorHeader { unsafe { p . cast () . into_mut () } }
};
}
