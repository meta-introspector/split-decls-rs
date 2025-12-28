macro_rules! deps {
    () => {
        Write!();
        Ready!();
    };
}

macro_rules! impl_1072 {
    () => {
        deps!();
        impl < W : AsyncWrite > BufWriter < W > { # [doc = " Creates a new `BufWriter` with a default buffer capacity. The default is currently 8 KB,"] # [doc = " but may change in the future."] pub fn new (inner : W) -> Self { Self :: with_capacity (DEFAULT_BUF_SIZE , inner) } # [doc = " Creates a new `BufWriter` with the specified buffer capacity."] pub fn with_capacity (cap : usize , inner : W) -> Self { Self { inner , buf : Vec :: with_capacity (cap) , written : 0 } } pub (super) fn flush_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { let mut this = self . project () ; let len = this . buf . len () ; let mut ret = Ok (()) ; while * this . written < len { match ready ! (this . inner . as_mut () . poll_write (cx , & this . buf [* this . written ..])) { Ok (0) => { ret = Err (io :: Error :: new (io :: ErrorKind :: WriteZero , "failed to write the buffered data" ,)) ; break ; } Ok (n) => * this . written += n , Err (e) => { ret = Err (e) ; break ; } } } if * this . written > 0 { this . buf . drain (.. * this . written) ; } * this . written = 0 ; Poll :: Ready (ret) } # [doc = " Write directly using `inner`, bypassing buffering"] pub (super) fn inner_poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { self . project () . inner . poll_write (cx , buf) } # [doc = " Write directly using `inner`, bypassing buffering"] pub (super) fn inner_poll_write_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { self . project () . inner . poll_write_vectored (cx , bufs) } }
    };
}

impl_1072!();