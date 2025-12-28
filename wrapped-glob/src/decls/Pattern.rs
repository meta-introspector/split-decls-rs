macro_rules! deps {
    () => {
        PatternToken!();
    };
}

macro_rules! Pattern {
    () => {
        deps!();
        # [doc = " A compiled Unix shell style pattern."] # [doc = ""] # [doc = " - `?` matches any single character."] # [doc = ""] # [doc = " - `*` matches any (possibly empty) sequence of characters."] # [doc = ""] # [doc = " - `**` matches the current directory and arbitrary"] # [doc = "   subdirectories. To match files in arbitrary subdirectories, use"] # [doc = "   `**/*`."] # [doc = ""] # [doc = "   This sequence **must** form a single path component, so both"] # [doc = "   `**a` and `b**` are invalid and will result in an error.  A"] # [doc = "   sequence of more than two consecutive `*` characters is also"] # [doc = "   invalid."] # [doc = ""] # [doc = " - `[...]` matches any character inside the brackets.  Character sequences"] # [doc = "   can also specify ranges of characters, as ordered by Unicode, so e.g."] # [doc = "   `[0-9]` specifies any character between 0 and 9 inclusive. An unclosed"] # [doc = "   bracket is invalid."] # [doc = ""] # [doc = " - `[!...]` is the negation of `[...]`, i.e. it matches any characters"] # [doc = "   **not** in the brackets."] # [doc = ""] # [doc = " - The metacharacters `?`, `*`, `[`, `]` can be matched by using brackets"] # [doc = "   (e.g. `[?]`).  When a `]` occurs immediately following `[` or `[!` then it"] # [doc = "   is interpreted as being part of, rather then ending, the character set, so"] # [doc = "   `]` and NOT `]` can be matched by `[]]` and `[!]]` respectively.  The `-`"] # [doc = "   character can be specified inside a character sequence pattern by placing"] # [doc = "   it at the start or the end, e.g. `[abc-]`."] # [derive (Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Default , Debug)] pub struct Pattern { original : String , tokens : Vec < PatternToken > , is_recursive : bool , # [doc = " A bool value that indicates whether the pattern contains any metacharacters."] # [doc = " We use this information for some fast path optimizations."] has_metachars : bool , }
    };
}

Pattern!();