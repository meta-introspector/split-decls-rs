macro_rules! impl_292 {
    () => {
        impl < R1 : AsyncRead , R2 : AsyncRead > AsyncRead for Chain < R1 , R2 > { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < Result < usize > > { let this = self . project () ; if ! * this . done_first { match ready ! (this . first . poll_read (cx , buf)) { Ok (0) if ! buf . is_empty () => * this . done_first = true , Ok (n) => return Poll :: Ready (Ok (n)) , Err (err) => return Poll :: Ready (Err (err)) , } } this . second . poll_read (cx , buf) } fn poll_read_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < Result < usize > > { let this = self . project () ; if ! * this . done_first { match ready ! (this . first . poll_read_vectored (cx , bufs)) { Ok (0) if ! bufs . is_empty () => * this . done_first = true , Ok (n) => return Poll :: Ready (Ok (n)) , Err (err) => return Poll :: Ready (Err (err)) , } } this . second . poll_read_vectored (cx , bufs) } }
    };
}

impl_292!()