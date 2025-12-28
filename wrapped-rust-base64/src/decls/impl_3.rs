macro_rules! deps {
    () => {
        Engine!();
        ChunkedEncoder!();
        Sink!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < 'e , E : Engine + ? Sized > ChunkedEncoder < 'e , E > { pub fn new (engine : & 'e E) -> ChunkedEncoder < 'e , E > { ChunkedEncoder { engine } } pub fn encode < S : Sink > (& self , bytes : & [u8] , sink : & mut S) -> Result < () , S :: Error > { const BUF_SIZE : usize = 1024 ; const CHUNK_SIZE : usize = BUF_SIZE / 4 * 3 ; let mut buf = [0 ; BUF_SIZE] ; for chunk in bytes . chunks (CHUNK_SIZE) { let mut len = self . engine . internal_encode (chunk , & mut buf) ; if chunk . len () != CHUNK_SIZE && self . engine . config () . encode_padding () { len += add_padding (len , & mut buf [len ..]) ; } sink . write_encoded_bytes (& buf [.. len]) ? ; } Ok (()) } }
    };
}

impl_3!()