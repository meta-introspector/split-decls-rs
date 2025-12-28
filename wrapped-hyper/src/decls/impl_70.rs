macro_rules! deps {
    () => {
        ReadBuf!();
        Read!();
        Compat!();
        Result!();
        Error!();
        ReadBufCursor!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        # [cfg (test)] impl < T > crate :: rt :: Read for Compat < T > where T : tokio :: io :: AsyncRead , { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , mut buf : crate :: rt :: ReadBufCursor < '_ > ,) -> Poll < Result < () , std :: io :: Error > > { let n = unsafe { let mut tbuf = tokio :: io :: ReadBuf :: uninit (buf . as_mut ()) ; match tokio :: io :: AsyncRead :: poll_read (self . p () , cx , & mut tbuf) { Poll :: Ready (Ok (())) => tbuf . filled () . len () , other => return other , } } ; unsafe { buf . advance (n) ; } Poll :: Ready (Ok (())) } }
    };
}

impl_70!();