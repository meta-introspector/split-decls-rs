macro_rules! deps {
    () => {
        EscapeError!();
    };
}

macro_rules! check_raw_c_str {
    () => {
        deps!();
        # [doc = " Check a raw C string literal for validity"] # [doc = ""] # [doc = " Takes the contents of a raw C string literal (without quotes)"] # [doc = " and produces a sequence of characters or errors,"] # [doc = " which are returned by invoking `callback`."] # [doc = " NOTE: Does no escaping, but produces errors for bare carriage return ('\\r')."] pub fn check_raw_c_str (src : & str , callback : impl FnMut (Range < usize > , Result < NonZero < char > , EscapeError >) ,) { CStr :: check_raw (src , callback) ; }
    };
}

check_raw_c_str!()