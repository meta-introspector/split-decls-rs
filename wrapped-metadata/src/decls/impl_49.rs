macro_rules! deps {
    () => {
        Column!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl Column { fn new (offset : usize , width : usize) -> Self { Self { offset , width } } }
    };
}

impl_49!()