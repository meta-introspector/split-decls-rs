macro_rules! deps {
    () => {
        Line!();
    };
}

macro_rules! LineReader {
    () => {
        deps!();
        # [doc = " Iterator over multi-line Base64 input."] # [derive (Clone)] struct LineReader < 'i > { # [doc = " Remaining linewrapped data to be processed."] remaining : & 'i [u8] , # [doc = " Line width."] line_width : Option < usize > , }
    };
}

LineReader!()