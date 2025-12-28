macro_rules! deps {
    () => {
        EscapeError!();
        Unescape!();
    };
}

macro_rules! unescape_char {
    () => {
        deps!();
        # [doc = " Unescape a char literal"] # [doc = ""] # [doc = " Takes the contents of a char literal (without quotes),"] # [doc = " and returns an unescaped char or an error."] # [inline] pub fn unescape_char (src : & str) -> Result < char , EscapeError > { str :: unescape_single (& mut src . chars ()) }
    };
}

unescape_char!();