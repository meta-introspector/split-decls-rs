macro_rules! deps {
    () => {
        Row!();
        File!();
    };
}

macro_rules! impl_465 {
    () => {
        deps!();
        impl Row { pub (crate) fn new (file : & 'static File , index : usize) -> Self { Self { file , index } } }
    };
}

impl_465!()