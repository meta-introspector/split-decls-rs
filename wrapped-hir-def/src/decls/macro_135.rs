macro_rules! deps {
    () => {
        AssocItemId!();
    };
}

macro_rules! macro_135 {
    () => {
        deps!();
        impl_from ! (FunctionId , ConstId , TypeAliasId for AssocItemId) ;
    };
}

macro_135!()