macro_rules! deps {
    () => {
        FileSnapshot!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < T : std :: fmt :: Debug > From < T > for FileSnapshot < T > { fn from (value : T) -> Self { FileSnapshot :: new (value) } }
    };
}

impl_8!();