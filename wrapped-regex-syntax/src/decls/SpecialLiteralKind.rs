macro_rules! SpecialLiteralKind {
    () => {
        # [doc = " The type of a special literal."] # [doc = ""] # [doc = " A special literal is a special escape sequence recognized by the regex"] # [doc = " parser, e.g., `\\f` or `\\n`."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub enum SpecialLiteralKind { # [doc = " Bell, spelled `\\a` (`\\x07`)."] Bell , # [doc = " Form feed, spelled `\\f` (`\\x0C`)."] FormFeed , # [doc = " Tab, spelled `\\t` (`\\x09`)."] Tab , # [doc = " Line feed, spelled `\\n` (`\\x0A`)."] LineFeed , # [doc = " Carriage return, spelled `\\r` (`\\x0D`)."] CarriageReturn , # [doc = " Vertical tab, spelled `\\v` (`\\x0B`)."] VerticalTab , # [doc = " Space, spelled `\\ ` (`\\x20`). Note that this can only appear when"] # [doc = " parsing in verbose mode."] Space , }
    };
}

SpecialLiteralKind!();