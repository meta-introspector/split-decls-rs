macro_rules! Loc {
    () => {
        # [doc = " A source code location used for error reporting."] # [derive (Clone , Copy , Debug , Default , PartialOrd , Ord , PartialEq , Eq)] pub (crate) struct Loc { # [doc = " The (1-based) line number."] pub (crate) line : usize , # [doc = " The (0-based) column offset."] pub (crate) char : usize , # [doc = " The (0-based) column offset when displayed."] pub (crate) display : usize , # [doc = " The (0-based) byte offset."] pub (crate) byte : usize , }
    };
}

Loc!()