macro_rules! deps {
    () => {
        Match!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < 'h > From < Match < 'h > > for & 'h [u8] { fn from (m : Match < 'h >) -> & 'h [u8] { m . as_bytes () } }
    };
}

impl_34!();