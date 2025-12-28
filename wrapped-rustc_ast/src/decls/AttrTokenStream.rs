macro_rules! deps {
    () => {
        TokenStream!();
        AttrTokenTree!();
    };
}

macro_rules! AttrTokenStream {
    () => {
        deps!();
        # [doc = " An `AttrTokenStream` is similar to a `TokenStream`, but with extra"] # [doc = " information about the tokens for attribute targets. This is used"] # [doc = " during expansion to perform early cfg-expansion, and to process attributes"] # [doc = " during proc-macro invocations."] # [derive (Clone , Debug , Default , Encodable , Decodable)] pub struct AttrTokenStream (pub Arc < Vec < AttrTokenTree > >) ;
    };
}

AttrTokenStream!()