macro_rules! deps {
    () => {
        TokenTreeCursor!();
        TokenStream!();
        TokenTree!();
    };
}

macro_rules! impl_454 {
    () => {
        deps!();
        impl TokenTreeCursor { # [inline] pub fn new (stream : TokenStream) -> Self { TokenTreeCursor { stream , index : 0 } } # [inline] pub fn curr (& self) -> Option < & TokenTree > { self . stream . get (self . index) } pub fn look_ahead (& self , n : usize) -> Option < & TokenTree > { self . stream . get (self . index + n) } # [inline] pub fn bump (& mut self) { self . index += 1 ; } # [inline] pub fn bump_to_end (& mut self) { self . index = self . stream . len () ; } }
    };
}

impl_454!()