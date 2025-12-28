macro_rules! deps {
    () => {
        AnsiGenericString!();
    };
}

macro_rules! AnsiByteString {
    () => {
        deps!();
        # [doc = " An `AnsiByteString` represents a formatted series of bytes.  Use"] # [doc = " `AnsiByteString` when styling text with an unknown encoding."] pub type AnsiByteString < 'a > = AnsiGenericString < 'a , [u8] > ;
    };
}

AnsiByteString!()