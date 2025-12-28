macro_rules! Trailers {
    () => {
        # [doc = " An iterator over trailers as parsed from a commit message body."] # [doc = ""] # [doc = " lines with parsing failures will be skipped"] pub struct Trailers < 'a > { pub (crate) cursor : & 'a [u8] , }
    };
}

Trailers!()