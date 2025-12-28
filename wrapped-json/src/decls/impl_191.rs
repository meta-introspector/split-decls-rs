macro_rules! deps {
    () => {
        Serializer!();
        PrettyFormatter!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl < 'a , W > Serializer < W , PrettyFormatter < 'a > > where W : io :: Write , { # [doc = " Creates a new JSON pretty print serializer."] # [inline] pub fn pretty (writer : W) -> Self { Serializer :: with_formatter (writer , PrettyFormatter :: new ()) } }
    };
}

impl_191!();