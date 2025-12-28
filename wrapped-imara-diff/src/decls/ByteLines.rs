macro_rules! deps {
    () => {
        TokenSource!();
    };
}

macro_rules! ByteLines {
    () => {
        deps!();
        # [doc = " A [`TokenSource`] that returns the lines of a byte slice as tokens. See [`byte_lines`]"] # [doc = " for details."] # [derive (Clone , Copy , PartialEq , Eq)] pub struct ByteLines < 'a > (& 'a [u8]) ;
    };
}

ByteLines!();