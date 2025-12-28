macro_rules! deps {
    () => {
        Match!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < 'h > From < Match < 'h > > for core :: ops :: Range < usize > { fn from (m : Match < 'h >) -> core :: ops :: Range < usize > { m . range () } }
    };
}

impl_35!()