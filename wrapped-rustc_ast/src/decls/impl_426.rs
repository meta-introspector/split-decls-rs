macro_rules! deps {
    () => {
        LazyAttrTokenStream!();
    };
}

macro_rules! impl_426 {
    () => {
        deps!();
        impl < S : SpanEncoder > Encodable < S > for LazyAttrTokenStream { fn encode (& self , _s : & mut S) { panic ! ("Attempted to encode LazyAttrTokenStream") ; } }
    };
}

impl_426!();