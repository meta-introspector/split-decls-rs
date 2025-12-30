// Generated macro for AnsiString (type)
macro_rules! Depcrate_displayAnsiString {
() => {
// Module: crate::display
// Provides: {"AnsiString"}
// Dependencies: {}
# [doc = " An ANSI String is a string coupled with the `Style` to display it"] # [doc = " in a terminal."] # [doc = ""] # [doc = " Although not technically a string itself, it can be turned into"] # [doc = " one with the `to_string` method."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use nu_ansi_term::AnsiString;"] # [doc = " use nu_ansi_term::Color::Red;"] # [doc = ""] # [doc = " let red_string = Red.paint(\"a red string\");"] # [doc = " println!(\"{}\", red_string);"] # [doc = " ```"] # [doc = ""] # [doc = " ```"] # [doc = " use nu_ansi_term::AnsiString;"] # [doc = ""] # [doc = " let plain_string = AnsiString::from(\"a plain string\");"] # [doc = " ```"] pub type AnsiString < 'a > = AnsiGenericString < 'a , str > ;
};
}
