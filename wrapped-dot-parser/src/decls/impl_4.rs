macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < 'a > ParseError < 'a > { fn expect_rule (expect : Vec < Rule > , found : Rule) -> Self { ParseError :: ExpectRule { expect , found } } fn missing_pair (parent : Pair < 'a , Rule > , expect : Vec < Rule >) -> Self { ParseError :: MissingPair { parent , expect } } }
    };
}

impl_4!()