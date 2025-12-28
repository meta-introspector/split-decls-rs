macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_1087 {
    () => {
        deps!();
        impl < T , U > AsyncRead for Chain < T , U > where T : AsyncRead , U : AsyncRead , { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { let this = self . project () ; if ! * this . done_first { match ready ! (this . first . poll_read (cx , buf) ?) { 0 if ! buf . is_empty () => * this . done_first = true , n => return Poll :: Ready (Ok (n)) , } } this . second . poll_read (cx , buf) } fn poll_read_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < io :: Result < usize > > { let this = self . project () ; if ! * this . done_first { let n = ready ! (this . first . poll_read_vectored (cx , bufs) ?) ; if n == 0 && bufs . iter () . any (| b | ! b . is_empty ()) { * this . done_first = true } else { return Poll :: Ready (Ok (n)) ; } } this . second . poll_read_vectored (cx , bufs) } }
    };
}

impl_1087!();