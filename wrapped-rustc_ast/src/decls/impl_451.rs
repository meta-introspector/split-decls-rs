macro_rules! deps {
    () => {
        TokenTree!();
        TokenStream!();
        TokenStreamIter!();
    };
}

macro_rules! impl_451 {
    () => {
        deps!();
        impl < 't > TokenStreamIter < 't > { fn new (stream : & 't TokenStream) -> Self { TokenStreamIter { stream , index : 0 } } pub fn peek (& self) -> Option < & 't TokenTree > { self . stream . 0 . get (self . index) } }
    };
}

impl_451!();