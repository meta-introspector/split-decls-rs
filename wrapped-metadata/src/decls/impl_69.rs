macro_rules! deps {
    () => {
        TypeIndex!();
        Row!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < 'a > Row < 'a > { pub (crate) fn new (index : & 'a TypeIndex , file : usize , pos : usize) -> Self { Self { index , file , pos } } }
    };
}

impl_69!()