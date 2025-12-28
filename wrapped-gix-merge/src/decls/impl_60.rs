macro_rules! deps {
    () => {
        Driver!();
        Platform!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        # [doc = " Access"] impl Platform { # [doc = " Return all drivers that this instance was initialized with."] # [doc = ""] # [doc = " They are sorted by [`name`](super::Driver::name) to support binary searches."] pub fn drivers (& self) -> & [super :: Driver] { & self . drivers } }
    };
}

impl_60!();