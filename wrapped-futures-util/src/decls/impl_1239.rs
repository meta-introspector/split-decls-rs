macro_rules! deps {
    () => {
        WriteAll!();
        Ready!();
    };
}

macro_rules! impl_1239 {
    () => {
        deps!();
        impl < W : AsyncWrite + ? Sized + Unpin > Future for WriteAll < '_ , W > { type Output = io :: Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { let this = & mut * self ; while ! this . buf . is_empty () { let n = ready ! (Pin :: new (& mut this . writer) . poll_write (cx , this . buf)) ? ; { let (_ , rest) = mem :: take (& mut this . buf) . split_at (n) ; this . buf = rest ; } if n == 0 { return Poll :: Ready (Err (io :: ErrorKind :: WriteZero . into ())) ; } } Poll :: Ready (Ok (())) } }
    };
}

impl_1239!()