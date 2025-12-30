// Generated macro for ErrorKind (enum)
macro_rules! DepcrateErrorKind {
() => {
// Module: crate
// Provides: {"ErrorKind"}
// Dependencies: {}
# [doc = " The kind of error that can occur when parsing a glob pattern."] # [derive (Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub enum ErrorKind { # [doc = " **DEPRECATED**."] # [doc = ""] # [doc = " This error used to occur for consistency with git's glob specification,"] # [doc = " but the specification now accepts all uses of `**`. When `**` does not"] # [doc = " appear adjacent to a path separator or at the beginning/end of a glob,"] # [doc = " it is now treated as two consecutive `*` patterns. As such, this error"] # [doc = " is no longer used."] InvalidRecursive , # [doc = " Occurs when a character class (e.g., `[abc]`) is not closed."] UnclosedClass , # [doc = " Occurs when a range in a character (e.g., `[a-z]`) is invalid. For"] # [doc = " example, if the range starts with a lexicographically larger character"] # [doc = " than it ends with."] InvalidRange (char , char) , # [doc = " Occurs when a `}` is found without a matching `{`."] UnopenedAlternates , # [doc = " Occurs when a `{` is found without a matching `}`."] UnclosedAlternates , # [doc = " **DEPRECATED**."] # [doc = ""] # [doc = " This error used to occur when an alternating group was nested inside"] # [doc = " another alternating group, e.g., `{{a,b},{c,d}}`. However, this is now"] # [doc = " supported and as such this error cannot occur."] NestedAlternates , # [doc = " Occurs when an unescaped '\\' is found at the end of a glob."] DanglingEscape , # [doc = " An error associated with parsing or compiling a regex."] Regex (String) , }
};
}
