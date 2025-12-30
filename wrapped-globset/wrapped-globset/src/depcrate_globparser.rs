// Generated macro for Parser (struct)
macro_rules! Depcrate_globParser {
() => {
// Module: crate::glob
// Provides: {"Parser"}
// Dependencies: {}
struct Parser < 'a > { # [doc = " The glob to parse."] glob : & 'a str , # [doc = " Marks the index in `stack` where the alternation started."] alternates_stack : Vec < usize > , # [doc = " The set of active alternation branches being parsed."] # [doc = " Tokens are added to the end of the last one."] branches : Vec < Tokens > , # [doc = " A character iterator over the glob pattern to parse."] chars : std :: iter :: Peekable < std :: str :: Chars < 'a > > , # [doc = " The previous character seen."] prev : Option < char > , # [doc = " The current character."] cur : Option < char > , # [doc = " Whether we failed to find a closing `]` for a character"] # [doc = " class. This can only be true when `GlobOptions::allow_unclosed_class`"] # [doc = " is enabled. When enabled, it is impossible to ever parse another"] # [doc = " character class with this glob. That's because classes cannot be"] # [doc = " nested *and* the only way this happens is when there is never a `]`."] # [doc = ""] # [doc = " We track this state so that we don't end up spending quadratic time"] # [doc = " trying to parse something like `[[[[[[[[[[[[[[[[[[[[[[[...`."] found_unclosed_class : bool , # [doc = " Glob options, which may influence parsing."] opts : & 'a GlobOptions , }
};
}
