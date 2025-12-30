// Generated macro for Indentation (struct)
macro_rules! Depcrate_writerIndentation {
() => {
// Module: crate::writer
// Provides: {"Indentation"}
// Dependencies: {}
# [derive (Clone)] pub (crate) struct Indentation { # [doc = " todo: this is an awkward fit as it has no impact on indentation logic, but it is"] # [doc = " only applicable when an indentation exists. Potentially refactor later"] should_line_break : bool , # [doc = " The character code to be used for indentations (e.g. ` ` or `\\t`)"] indent_char : u8 , # [doc = " How many instances of the indent character ought to be used for each level of indentation"] indent_size : usize , # [doc = " Used as a cache for the bytes used for indentation"] indents : Vec < u8 > , # [doc = " The current amount of indentation"] current_indent_len : usize , }
};
}
