// Generated macro for Shlex (struct)
macro_rules! Depcrate_bytesShlex {
() => {
// Module: crate::bytes
// Provides: {"Shlex"}
// Dependencies: {}
# [doc = " An iterator that takes an input byte string and splits it into the words using the same syntax as"] # [doc = " the POSIX shell."] pub struct Shlex < 'a > { in_iter : core :: slice :: Iter < 'a , u8 > , # [doc = " The number of newlines read so far, plus one."] pub line_no : usize , # [doc = " An input string is erroneous if it ends while inside a quotation or right after an"] # [doc = " unescaped backslash.  Since Iterator does not have a mechanism to return an error, if that"] # [doc = " happens, Shlex just throws out the last token, ends the iteration, and sets 'had_error' to"] # [doc = " true; best to check it after you're done iterating."] pub had_error : bool , }
};
}
