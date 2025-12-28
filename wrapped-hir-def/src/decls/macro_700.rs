macro_rules! deps {
    () => {
        GeneralConstId!();
    };
}

macro_rules! macro_700 {
    () => {
        deps!();
        impl_from ! (ConstId , StaticId for GeneralConstId) ;
    };
}

macro_700!();