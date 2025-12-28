macro_rules! deps {
    () => {
        Range!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl Range { # [doc = " A convenience method to create a new instance of `name`."] pub fn new (name : & 'static str) -> Self { Range { name } } }
    };
}

impl_108!()