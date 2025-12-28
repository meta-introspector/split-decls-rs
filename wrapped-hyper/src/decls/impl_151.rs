macro_rules! deps {
    () => {
        Protocol!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        # [cfg (feature = "http2")] impl < 'a > From < & 'a str > for Protocol { fn from (value : & 'a str) -> Self { Self { inner : h2 :: ext :: Protocol :: from (value) , } } }
    };
}

impl_151!();