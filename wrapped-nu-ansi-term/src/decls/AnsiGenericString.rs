macro_rules! deps {
    () => {
        Style!();
        OSControl!();
        AnsiString!();
        AnsiByteString!();
    };
}

macro_rules! AnsiGenericString {
    () => {
        deps!();
        # [doc = " An `AnsiGenericString` includes a generic string type and a `Style` to"] # [doc = " display that string.  `AnsiString` and `AnsiByteString` are aliases for"] # [doc = " this type on `str` and `\\[u8]`, respectively."] # [derive (Eq , PartialEq , Debug)] pub struct AnsiGenericString < 'a , S : 'a + ToOwned + ? Sized > where < S as ToOwned > :: Owned : fmt :: Debug , { pub (crate) style : Style , pub (crate) string : Cow < 'a , S > , oscontrol : Option < OSControl < 'a , S > > , }
    };
}

AnsiGenericString!();