macro_rules! deps {
    () => {
        ByteLines!();
        TokenSource!();
    };
}

macro_rules! Lines {
    () => {
        deps!();
        # [doc = " A [`TokenSource`] that returns the lines of a `str` as tokens. See [`lines`] for"] # [doc = " details."] # [derive (Clone , Copy , PartialEq , Eq)] pub struct Lines < 'a > (ByteLines < 'a >) ;
    };
}

Lines!();