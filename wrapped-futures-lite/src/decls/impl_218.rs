macro_rules! impl_218 {
    () => {
        impl < R : AsyncRead > AsyncBufRead for BufReader < R > { fn poll_fill_buf < 'a > (self : Pin < & 'a mut Self > , cx : & mut Context < '_ >) -> Poll < Result < & 'a [u8] > > { let mut this = self . project () ; if * this . pos >= * this . cap { debug_assert ! (* this . pos == * this . cap) ; * this . cap = ready ! (this . inner . as_mut () . poll_read (cx , this . buf)) ? ; * this . pos = 0 ; } Poll :: Ready (Ok (& this . buf [* this . pos .. * this . cap])) } fn consume (self : Pin < & mut Self > , amt : usize) { let this = self . project () ; * this . pos = cmp :: min (* this . pos + amt , * this . cap) ; } }
    };
}

impl_218!()