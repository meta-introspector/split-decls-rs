macro_rules! WideChar {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] struct WideChar { # [doc = " Start offset of a character inside a line, zero-based."] start : TextSize , # [doc = " End offset of a character inside a line, zero-based."] end : TextSize , }
    };
}

WideChar!();