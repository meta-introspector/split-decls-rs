macro_rules! deps {
    () => {
        Error!();
        StackError!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl Error for StackError { fn source (& self) -> Option < & (dyn Error + 'static) > { match * self { StackError :: ExceedsMaximumSize (_) => None , StackError :: IoError (ref e) => Some (e) , } } }
    };
}

impl_63!();