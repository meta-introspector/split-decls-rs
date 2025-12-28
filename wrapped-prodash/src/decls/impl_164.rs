macro_rules! deps {
    () => {
        DoOrDiscard!();
        Either!();
        NestedProgress!();
        Discard!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl < T > From < Option < T > > for DoOrDiscard < T > where T : NestedProgress , { fn from (p : Option < T >) -> Self { match p { Some (p) => DoOrDiscard (Either :: Left (p)) , None => DoOrDiscard (Either :: Right (Discard)) , } } }
    };
}

impl_164!();