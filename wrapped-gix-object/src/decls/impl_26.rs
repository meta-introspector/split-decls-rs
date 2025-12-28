macro_rules! deps {
    () => {
        Tree!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl Tree { # [doc = " Return an empty tree which serializes to a well-known hash"] pub fn empty () -> Self { Tree { entries : Vec :: new () } } }
    };
}

impl_26!()