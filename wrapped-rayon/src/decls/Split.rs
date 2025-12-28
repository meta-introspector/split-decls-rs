macro_rules! Split {
    () => {
        # [doc = " Parallel iterator over substrings separated by a pattern"] # [derive (Debug , Clone)] pub struct Split < 'ch , P : Pattern > { chars : & 'ch str , separator : P , }
    };
}

Split!();