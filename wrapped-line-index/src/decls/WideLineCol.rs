macro_rules! deps {
    () => {
        WideEncoding!();
    };
}

macro_rules! WideLineCol {
    () => {
        deps!();
        # [doc = " `(line, column)` information in wide encodings."] # [doc = ""] # [doc = " See [`WideEncoding`] for the kinds of wide encodings available."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct WideLineCol { # [doc = " Zero-based."] pub line : u32 , # [doc = " Zero-based."] pub col : u32 , }
    };
}

WideLineCol!();