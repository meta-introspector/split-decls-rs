macro_rules! deps {
    () => {
        WideChar!();
    };
}

macro_rules! LineIndex {
    () => {
        deps!();
        # [doc = " Maps flat [`TextSize`] offsets to/from `(line, column)` representation."] # [derive (Debug , Clone , PartialEq , Eq)] pub struct LineIndex { # [doc = " Offset the beginning of each line (except the first, which always has offset 0)."] newlines : Box < [TextSize] > , # [doc = " List of non-ASCII characters on each line."] line_wide_chars : IntMap < u32 , Box < [WideChar] > > , # [doc = " The length of the entire text."] len : TextSize , }
    };
}

LineIndex!();