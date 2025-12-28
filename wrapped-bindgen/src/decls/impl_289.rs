macro_rules! deps {
    () => {
        CppStruct!();
    };
}

macro_rules! impl_289 {
    () => {
        deps!();
        impl Eq for CppStruct { }
    };
}

impl_289!();