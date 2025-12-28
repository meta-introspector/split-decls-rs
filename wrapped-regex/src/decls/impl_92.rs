macro_rules! deps {
    () => {
        Match!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < 'h > From < Match < 'h > > for & 'h str { fn from (m : Match < 'h >) -> & 'h str { m . as_str () } }
    };
}

impl_92!();