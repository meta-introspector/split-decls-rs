// Generated macro for ANSIString (type)
macro_rules! Depcrate_displayANSIString {
() => {
// Module: crate::display
// Provides: {"ANSIString"}
// Dependencies: {}
# [doc = " An ANSI String is a string coupled with the `Style` to display it"] # [doc = " in a terminal."] # [doc = ""] # [doc = " Although not technically a string itself, it can be turned into"] # [doc = " one with the `to_string` method."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use ansiterm::ANSIString;"] # [doc = " use ansiterm::Colour::Red;"] # [doc = ""] # [doc = " let red_string = Red.paint(\"a red string\");"] # [doc = " println!(\"{}\", red_string);"] # [doc = " ```"] # [doc = ""] # [doc = " ```"] # [doc = " use ansiterm::ANSIString;"] # [doc = ""] # [doc = " let plain_string = ANSIString::from(\"a plain string\");"] # [doc = " assert_eq!(&*plain_string, \"a plain string\");"] # [doc = " ```"] pub type ANSIString < 'a > = ANSIGenericString < 'a , str > ;
};
}
