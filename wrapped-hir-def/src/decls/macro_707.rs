macro_rules! deps {
    () => {
        AssocItemId!();
    };
}

macro_rules! macro_707 {
    () => {
        deps!();
        impl_from ! (FunctionId , ConstId , TypeAliasId for AssocItemId) ;
    };
}

macro_707!()