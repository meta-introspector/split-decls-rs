macro_rules! deps {
    () => {
        GeneralConstId!();
    };
}

macro_rules! macro_128 {
    () => {
        deps!();
        impl_from ! (ConstId , StaticId for GeneralConstId) ;
    };
}

macro_128!()