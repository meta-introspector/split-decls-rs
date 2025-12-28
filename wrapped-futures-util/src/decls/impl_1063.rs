macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_1063 {
    () => {
        deps!();
        impl < R : AsyncRead > AsyncRead for BufReader < R > { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { if self . pos == self . cap && buf . len () >= self . buffer . len () { let res = ready ! (self . as_mut () . project () . inner . poll_read (cx , buf)) ; self . discard_buffer () ; return Poll :: Ready (res) ; } let mut rem = ready ! (self . as_mut () . poll_fill_buf (cx)) ? ; let nread = rem . read (buf) ? ; self . consume (nread) ; Poll :: Ready (Ok (nread)) } fn poll_read_vectored (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < io :: Result < usize > > { let total_len = bufs . iter () . map (| b | b . len ()) . sum :: < usize > () ; if self . pos == self . cap && total_len >= self . buffer . len () { let res = ready ! (self . as_mut () . project () . inner . poll_read_vectored (cx , bufs)) ; self . discard_buffer () ; return Poll :: Ready (res) ; } let mut rem = ready ! (self . as_mut () . poll_fill_buf (cx)) ? ; let nread = rem . read_vectored (bufs) ? ; self . consume (nread) ; Poll :: Ready (Ok (nread)) } }
    };
}

impl_1063!()