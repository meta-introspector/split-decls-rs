// Generated macro for FormatterExt (trait)
macro_rules! Depcrate_extFormatterExt {
() => {
// Module: crate::ext
// Provides: {"FormatterExt"}
// Dependencies: {}
# [doc = " An extension trait for [`core::fmt::Formatter`]."] pub trait FormatterExt : sealed :: Sealed { # [doc = " Writes the given arguments to the formatter, padding them with the given width. If `width`"] # [doc = " is incorrect, the resulting output will not be the requested width."] fn pad_with_width (& mut self , width : usize , args : Arguments < '_ >) -> Result ; }
};
}
