macro_rules! deps {
    () => {
        Ready!();
        Aborted!();
        Pending!();
    };
}

macro_rules! impl_1106 {
    () => {
        deps!();
        impl < R , W > Future for CopyBufAbortable < '_ , R , W > where R : AsyncBufRead , W : AsyncWrite + Unpin + Sized , { type Output = Result < Result < u64 , Aborted > , io :: Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { if this . inner . aborted . load (Ordering :: Relaxed) { return Poll :: Ready (Ok (Err (Aborted))) ; } let buffer = ready_or_break ! (this . reader . as_mut () . poll_fill_buf (cx)) ? ; if buffer . is_empty () { ready_or_break ! (Pin :: new (& mut this . writer) . poll_flush (cx)) ? ; return Poll :: Ready (Ok (Ok (* this . amt))) ; } let i = ready_or_break ! (Pin :: new (& mut this . writer) . poll_write (cx , buffer)) ? ; if i == 0 { return Poll :: Ready (Err (io :: ErrorKind :: WriteZero . into ())) ; } * this . amt += i as u64 ; this . reader . as_mut () . consume (i) ; } this . inner . waker . register (cx . waker ()) ; if this . inner . aborted . load (Ordering :: Relaxed) { return Poll :: Ready (Ok (Err (Aborted))) ; } Poll :: Pending } }
    };
}

impl_1106!();