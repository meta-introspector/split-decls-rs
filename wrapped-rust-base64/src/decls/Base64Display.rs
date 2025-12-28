macro_rules! deps {
    () => {
        Engine!();
        ChunkedEncoder!();
    };
}

macro_rules! Base64Display {
    () => {
        deps!();
        # [doc = " A convenience wrapper for base64'ing bytes into a format string without heap allocation."] pub struct Base64Display < 'a , 'e , E : Engine > { bytes : & 'a [u8] , chunked_encoder : ChunkedEncoder < 'e , E > , }
    };
}

Base64Display!();