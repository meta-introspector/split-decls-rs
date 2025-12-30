// Generated macro for macro_7058 (macro)
macro_rules! Depcrate_methodsmacro_7058 {
() => {
// Module: crate::methods
// Provides: {"macro_7058"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Checks for jumps to the start of a stream that implements `Seek`"] # [doc = " and uses the `seek` method providing `Start` as parameter."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " Readability. There is a specific method that was implemented for"] # [doc = " this exact scenario."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::io;"] # [doc = " fn foo<T: io::Seek>(t: &mut T) {"] # [doc = "     t.seek(io::SeekFrom::Start(0));"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::io;"] # [doc = " fn foo<T: io::Seek>(t: &mut T) {"] # [doc = "     t.rewind();"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.67.0"] pub SEEK_TO_START_INSTEAD_OF_REWIND , complexity , "jumping to the start of stream using `seek` method" }
};
}
