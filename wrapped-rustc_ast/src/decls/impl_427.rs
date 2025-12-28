macro_rules! deps {
    () => {
        LazyAttrTokenStream!();
    };
}

macro_rules! impl_427 {
    () => {
        deps!();
        impl < D : SpanDecoder > Decodable < D > for LazyAttrTokenStream { fn decode (_d : & mut D) -> Self { panic ! ("Attempted to decode LazyAttrTokenStream") ; } }
    };
}

impl_427!();