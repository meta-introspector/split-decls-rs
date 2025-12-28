macro_rules! deps {
    () => {
        Base64Display!();
        ChunkedEncoder!();
        Engine!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < 'a , 'e , E : Engine > Base64Display < 'a , 'e , E > { # [doc = " Create a `Base64Display` with the provided engine."] pub fn new (bytes : & 'a [u8] , engine : & 'e E) -> Base64Display < 'a , 'e , E > { Base64Display { bytes , chunked_encoder : ChunkedEncoder :: new (engine) , } } }
    };
}

impl_10!()