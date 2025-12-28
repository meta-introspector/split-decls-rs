macro_rules! deps {
    () => {
        DefWithBodyId!();
    };
}

macro_rules! macro_131 {
    () => {
        deps!();
        impl_from ! (FunctionId , ConstId , StaticId for DefWithBodyId) ;
    };
}

macro_131!()