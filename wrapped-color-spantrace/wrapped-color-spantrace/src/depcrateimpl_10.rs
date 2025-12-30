// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl Theme { # [doc = " Create blank theme"] pub fn new () -> Self { Self :: default () } # [doc = " A theme for a dark background. This is the default"] pub fn dark () -> Self { Self { file : style () . purple () , line_number : style () . purple () , active_line : style () . white () . bold () , target : style () . bright_red () , fields : style () . bright_cyan () , } } # [doc = " A theme for a light background"] pub fn light () -> Self { Self { file : style () . purple () , line_number : style () . purple () , target : style () . red () , fields : style () . blue () , active_line : style () . bold () , } } # [doc = " Styles printed paths"] pub fn file (mut self , style : Style) -> Self { self . file = style ; self } # [doc = " Styles the line number of a file"] pub fn line_number (mut self , style : Style) -> Self { self . line_number = style ; self } # [doc = " Styles the target (i.e. the module and function name, and so on)"] pub fn target (mut self , style : Style) -> Self { self . target = style ; self } # [doc = " Styles fields associated with a the `tracing::Span`."] pub fn fields (mut self , style : Style) -> Self { self . fields = style ; self } # [doc = " Styles the selected line of displayed code"] pub fn active_line (mut self , style : Style) -> Self { self . active_line = style ; self } }
};
}
