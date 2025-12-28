macro_rules! deps {
    () => {
        Response!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl Response < '_ > { # [inline] pub fn status (& self) -> http :: StatusCode { self . 0 . status () } # [inline] pub fn version (& self) -> http :: Version { self . 0 . version () } # [inline] pub fn headers (& self) -> & http :: HeaderMap { self . 0 . headers () } }
    };
}

impl_147!()