macro_rules! deps {
    () => {
        Read!();
        Compat!();
        Result!();
        ReadBuf!();
        Error!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < T > tokio :: io :: AsyncRead for Compat < T > where T : crate :: rt :: Read , { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , tbuf : & mut tokio :: io :: ReadBuf < '_ > ,) -> Poll < Result < () , std :: io :: Error > > { let init = tbuf . initialized () . len () ; let filled = tbuf . filled () . len () ; let (new_init , new_filled) = unsafe { let mut buf = crate :: rt :: ReadBuf :: uninit (tbuf . inner_mut ()) ; buf . set_init (init) ; buf . set_filled (filled) ; match crate :: rt :: Read :: poll_read (self . p () , cx , buf . unfilled ()) { Poll :: Ready (Ok (())) => (buf . init_len () , buf . len ()) , other => return other , } } ; let n_init = new_init - init ; unsafe { tbuf . assume_init (n_init) ; tbuf . set_filled (new_filled) ; } Poll :: Ready (Ok (())) } }
    };
}

impl_68!()