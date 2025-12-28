macro_rules! deps {
    () => {
        ValueParser!();
    };
}

macro_rules! _AnonymousValueParser {
    () => {
        deps!();
        # [doc = " Unstable [`ValueParser`]"] # [doc = ""] # [doc = " Implementation may change to more specific instance in the future"] # [doc (hidden)] # [derive (Debug)] pub struct _AnonymousValueParser (ValueParser) ;
    };
}

_AnonymousValueParser!();