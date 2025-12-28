macro_rules! deps {
    () => {
        Decoder!();
        DecoderImpl!();
        Config!();
        Reader!();
    };
}

macro_rules! impl_273 {
    () => {
        deps!();
        impl < R : Reader , C : Config , Context > DecoderImpl < R , C , Context > { # [doc = " Construct a new Decoder"] pub fn new (reader : R , config : C , context : Context) -> DecoderImpl < R , C , Context > { DecoderImpl { reader , config , bytes_read : 0 , context , } } }
    };
}

impl_273!()