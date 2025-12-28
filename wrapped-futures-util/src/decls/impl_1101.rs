macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_1101 {
    () => {
        deps!();
        impl < R , W > Future for CopyBuf < '_ , R , W > where R : AsyncBufRead , W : AsyncWrite + Unpin + ? Sized , { type Output = io :: Result < u64 > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { let buffer = ready ! (this . reader . as_mut () . poll_fill_buf (cx)) ? ; if buffer . is_empty () { ready ! (Pin :: new (& mut this . writer) . poll_flush (cx)) ? ; return Poll :: Ready (Ok (* this . amt)) ; } let i = ready ! (Pin :: new (& mut this . writer) . poll_write (cx , buffer)) ? ; if i == 0 { return Poll :: Ready (Err (io :: ErrorKind :: WriteZero . into ())) ; } * this . amt += i as u64 ; this . reader . as_mut () . consume (i) ; } } }
    };
}

impl_1101!();