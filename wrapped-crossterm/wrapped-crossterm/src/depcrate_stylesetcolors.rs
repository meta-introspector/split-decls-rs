// Generated macro for SetColors (struct)
macro_rules! Depcrate_styleSetColors {
() => {
// Module: crate::style
// Provides: {"SetColors"}
// Dependencies: {}
# [doc = " A command that optionally sets the foreground and/or background color."] # [doc = ""] # [doc = " For example:"] # [doc = " ```no_run"] # [doc = " use std::io::{stdout, Write};"] # [doc = ""] # [doc = " use crossterm::execute;"] # [doc = " use crossterm::style::{Color::{Green, Black}, Colors, Print, SetColors};"] # [doc = ""] # [doc = " execute!("] # [doc = "     stdout(),"] # [doc = "     SetColors(Colors::new(Green, Black)),"] # [doc = "     Print(\"Hello, world!\".to_string()),"] # [doc = " ).unwrap();"] # [doc = " ```"] # [doc = ""] # [doc = " See [`Colors`](struct.Colors.html) for more info."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " Commands must be executed/queued for execution otherwise they do nothing."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct SetColors (pub Colors) ;
};
}
