macro_rules! deps {
    () => {
        Color!();
    };
}

macro_rules! ChangeSet {
    () => {
        deps!();
        # [doc = " The changes that are implied by a tag."] # [derive (Debug , PartialEq , Default)] pub struct ChangeSet { # [doc = " If it is `Some`, then the foreground color has to be changed."] pub foreground : Option < Color > , # [doc = " If it is `Some`, then the background color has to be changed."] pub background : Option < Color > , # [doc = " If it is `true`, then the bold attribute has to be set (or unset for a close tag)."] pub bold : bool , # [doc = " If it is `true`, then the dim attribute has to be set (or unset for a close tag)."] pub dim : bool , # [doc = " If it is `true`, then the underline attribute has to be set (or unset for a close tag)."] pub underline : bool , # [doc = " If it is `true`, then the italics attribute has to be set (or unset for a close tag)."] pub italics : bool , # [doc = " If it is `true`, then the blink attribute has to be set (or unset for a close tag)."] pub blink : bool , # [doc = " If it is `true`, then the strike attribute has to be set (or unset for a close tag)."] pub strike : bool , # [doc = " If it is `true`, then the reverse attribute has to be set (or unset for a close tag)."] pub reverse : bool , # [doc = " If it is `true`, then the conceal attribute has to be set (or unset for a close tag)."] pub conceal : bool , }
    };
}

ChangeSet!();