macro_rules! deps {
    () => {
        ErrorKind!();
        Error!();
        Err!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < T > Err < (T , ErrorKind) > { # [doc = " Maps `Err<(T, ErrorKind)>` to `Err<(U, ErrorKind)>` with the given `F: T -> U`"] pub fn map_input < U , F > (self , f : F) -> Err < (U , ErrorKind) > where F : FnOnce (T) -> U , { match self { Err :: Incomplete (n) => Err :: Incomplete (n) , Err :: Failure ((input , k)) => Err :: Failure ((f (input) , k)) , Err :: Error ((input , k)) => Err :: Error ((f (input) , k)) , } } }
    };
}

impl_149!()