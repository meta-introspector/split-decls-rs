// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let mut frames = vec ! [] ; backtrace :: trace (| frame | { let converted = BacktraceFrame :: from (frame . clone ()) ; frames . push (converted) ; true }) ; let mut manual = Backtrace :: from (frames) ; manual . resolve () ; let frames = manual . frames () ; let mut output = String :: new () ; for frame in frames { writeln ! (output , "{:?}" , frame . ip ()) . unwrap () ; writeln ! (output , "{:?}" , frame . symbol_address ()) . unwrap () ; writeln ! (output , "{:?}" , frame . module_base_address ()) . unwrap () ; writeln ! (output , "{:?}" , frame . symbols ()) . unwrap () ; } drop (output) ; }
};
}
