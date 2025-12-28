macro_rules! deps {
    () => {
        SimpleBody!();
    };
}

macro_rules! EnumVariantBody {
    () => {
        deps!();
        pub type EnumVariantBody = SimpleBody ;
    };
}

EnumVariantBody!();