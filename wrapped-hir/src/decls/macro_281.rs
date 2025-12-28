macro_rules! deps {
    () => {
        Const!();
        Variant!();
        Static!();
        DefWithBody!();
        Function!();
    };
}

macro_rules! macro_281 {
    () => {
        deps!();
        impl_from ! (Function , Const , Static , Variant for DefWithBody) ;
    };
}

macro_281!()