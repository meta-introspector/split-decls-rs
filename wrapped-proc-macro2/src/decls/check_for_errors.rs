macro_rules! deps {
    () => {
        Mode!();
        EscapeError!();
    };
}

macro_rules! check_for_errors {
    () => {
        deps!();
        # [doc = " Check a literal only for errors"] # [doc = ""] # [doc = " Takes the contents of a literal (without quotes)"] # [doc = " and produces a sequence of only errors,"] # [doc = " which are returned by invoking `error_callback`."] # [doc = ""] # [doc = " NB Does not produce any output other than errors"] pub fn check_for_errors (src : & str , mode : Mode , mut error_callback : impl FnMut (Range < usize > , EscapeError) ,) { match mode { Mode :: Char => { let mut chars = src . chars () ; if let Err (e) = str :: unescape_single (& mut chars) { error_callback (0 .. (src . len () - chars . as_str () . len ()) , e) ; } } Mode :: Byte => { let mut chars = src . chars () ; if let Err (e) = < [u8] > :: unescape_single (& mut chars) { error_callback (0 .. (src . len () - chars . as_str () . len ()) , e) ; } } Mode :: Str => unescape_str (src , | range , res | { if let Err (e) = res { error_callback (range , e) ; } }) , Mode :: ByteStr => unescape_byte_str (src , | range , res | { if let Err (e) = res { error_callback (range , e) ; } }) , Mode :: CStr => unescape_c_str (src , | range , res | { if let Err (e) = res { error_callback (range , e) ; } }) , Mode :: RawStr => check_raw_str (src , | range , res | { if let Err (e) = res { error_callback (range , e) ; } }) , Mode :: RawByteStr => check_raw_byte_str (src , | range , res | { if let Err (e) = res { error_callback (range , e) ; } }) , Mode :: RawCStr => check_raw_c_str (src , | range , res | { if let Err (e) = res { error_callback (range , e) ; } }) , } }
    };
}

check_for_errors!()