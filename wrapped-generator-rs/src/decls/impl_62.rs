macro_rules! deps {
    () => {
        StackError!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl Display for StackError { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { match * self { StackError :: ExceedsMaximumSize (size) => write ! (fmt , "Requested more than max size of {size} bytes for a stack") , StackError :: IoError (ref e) => e . fmt (fmt) , } } }
    };
}

impl_62!();