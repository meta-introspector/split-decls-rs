macro_rules! deps {
    () => {
        Write!();
        Result!();
        Error!();
        Compat!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < T > tokio :: io :: AsyncWrite for Compat < T > where T : crate :: rt :: Write , { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < Result < usize , std :: io :: Error > > { crate :: rt :: Write :: poll_write (self . p () , cx , buf) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , std :: io :: Error > > { crate :: rt :: Write :: poll_flush (self . p () , cx) } fn poll_shutdown (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Result < () , std :: io :: Error > > { crate :: rt :: Write :: poll_shutdown (self . p () , cx) } fn is_write_vectored (& self) -> bool { crate :: rt :: Write :: is_write_vectored (& self . 0) } fn poll_write_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [std :: io :: IoSlice < '_ >] ,) -> Poll < Result < usize , std :: io :: Error > > { crate :: rt :: Write :: poll_write_vectored (self . p () , cx , bufs) } }
    };
}

impl_69!()