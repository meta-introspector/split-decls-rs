macro_rules! deps {
    () => {
        DefWithBody!();
        Const!();
        Variant!();
        Static!();
        Function!();
    };
}

macro_rules! macro_281 {
    () => {
        deps!();
        impl_from ! (Function , Const , Static , Variant for DefWithBody) ;
    };
}

macro_281!();