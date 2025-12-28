macro_rules! deps {
    () => {
        Const!();
        Static!();
        Variant!();
        DefWithBody!();
        Function!();
    };
}

macro_rules! macro_65 {
    () => {
        deps!();
        impl_from ! (Function , Const , Static , Variant for DefWithBody) ;
    };
}

macro_65!()