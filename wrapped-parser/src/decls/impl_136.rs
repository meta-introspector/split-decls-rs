macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl Display for Error { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { Self :: Syntax { message , .. } => f . write_str (message) , Self :: MissingQueryRoot { .. } => f . write_str ("schema definition is missing query root") , Self :: MultipleRoots { root , .. } => { write ! (f , "multiple {} roots in schema definition" , root) } Self :: MultipleOperations { .. } => f . write_str ("document contains multiple operations") , Self :: OperationDuplicated { operation , .. } => { write ! (f , "operation {} is defined twice" , operation) } Self :: FragmentDuplicated { fragment , .. } => { write ! (f , "fragment {} is defined twice" , fragment) } Self :: MissingOperation => f . write_str ("document does not contain an operation") , Self :: RecursionLimitExceeded => f . write_str ("recursion limit exceeded.") , } } }
    };
}

impl_136!()