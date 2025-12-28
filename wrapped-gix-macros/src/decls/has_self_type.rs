macro_rules! has_self_type {
    () => {
        fn has_self_type (input : & FnArg) -> bool { match input { FnArg :: Receiver (_) => true , FnArg :: Typed (PatType { ty , .. }) => contains_self_type (ty) , } }
    };
}

has_self_type!()