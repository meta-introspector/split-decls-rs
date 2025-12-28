macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_948 {
    () => {
        deps!();
        # [allow (clippy :: from_over_into)] impl < 'a > Into < Name > for & 'a [u8 ; 8] { fn into (self) -> Name { Name :: Short (* self) } }
    };
}

impl_948!();