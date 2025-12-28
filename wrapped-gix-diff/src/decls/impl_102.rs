macro_rules! deps {
    () => {
        Pipeline!();
        Driver!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        # [doc = " Access"] impl Pipeline { # [doc = " Return all drivers that this instance was initialized with."] # [doc = ""] # [doc = " They are sorted by [`name`](Driver::name) to support binary searches."] pub fn drivers (& self) -> & [super :: Driver] { & self . drivers } }
    };
}

impl_102!();