macro_rules! deps {
    () => {
        LineEnding!();
    };
}

macro_rules! LineWrapper {
    () => {
        deps!();
        # [doc = " Helper for wrapping Base64 at a given line width."] # [derive (Debug)] struct LineWrapper { # [doc = " Number of bytes remaining in the current line."] remaining : usize , # [doc = " Column at which Base64 should be wrapped."] width : usize , # [doc = " Newline characters to use at the end of each line."] ending : LineEnding , }
    };
}

LineWrapper!()