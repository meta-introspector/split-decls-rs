macro_rules! deps {
    () => {
        Err!();
        Error!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl < T > Err < error :: Error < T > > { # [doc = " Maps `Err<error::Error<T>>` to `Err<error::Error<U>>` with the given `F: T -> U`"] pub fn map_input < U , F > (self , f : F) -> Err < error :: Error < U > > where F : FnOnce (T) -> U , { match self { Err :: Incomplete (n) => Err :: Incomplete (n) , Err :: Failure (error :: Error { input , code }) => Err :: Failure (error :: Error { input : f (input) , code , }) , Err :: Error (error :: Error { input , code }) => Err :: Error (error :: Error { input : f (input) , code , }) , } } }
    };
}

impl_150!()