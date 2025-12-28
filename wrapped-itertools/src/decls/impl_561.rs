macro_rules! deps {
    () => {
        WithPosition!();
    };
}

macro_rules! impl_561 {
    () => {
        deps!();
        impl < I > fmt :: Debug for WithPosition < I > where I : Iterator , Peekable < Fuse < I > > : fmt :: Debug , { debug_fmt_fields ! (WithPosition , handled_first , peekable) ; }
    };
}

impl_561!()