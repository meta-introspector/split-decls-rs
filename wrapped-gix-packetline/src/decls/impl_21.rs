macro_rules! deps {
    () => {
        TextRef!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < 'a > From < & 'a [u8] > for TextRef < 'a > { fn from (d : & 'a [u8]) -> Self { let d = if d [d . len () - 1] == b'\n' { & d [.. d . len () - 1] } else { d } ; TextRef (d) } }
    };
}

impl_21!();