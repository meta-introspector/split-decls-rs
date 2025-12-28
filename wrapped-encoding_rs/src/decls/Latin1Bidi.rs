macro_rules! Latin1Bidi {
    () => {
        # [doc = " Classification of text as Latin1 (all code points are below U+0100),"] # [doc = " left-to-right with some non-Latin1 characters or as containing at least"] # [doc = " some right-to-left characters."] # [must_use] # [derive (Debug , PartialEq , Eq)] # [repr (C)] pub enum Latin1Bidi { # [doc = " Every character is below U+0100."] Latin1 = 0 , # [doc = " There is at least one character that's U+0100 or higher, but there"] # [doc = " are no right-to-left characters."] LeftToRight = 1 , # [doc = " There is at least one right-to-left character."] Bidi = 2 , }
    };
}

Latin1Bidi!();