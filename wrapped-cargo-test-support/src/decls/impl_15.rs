macro_rules! deps {
    () => {
        WildStr!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < 'a > WildStr < 'a > { fn new (line : & 'a str) -> WildStr < 'a > { WildStr { has_meta : line . contains ("[..]") , line , } } }
    };
}

impl_15!()