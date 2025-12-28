macro_rules! deps {
    () => {
        Adhoc!();
        Error!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl Adhoc { # [cold] pub fn new < M > (self , message : M) -> Error where M : Display + Debug + Send + Sync + 'static , { Error :: construct_from_adhoc (message , backtrace ! ()) } }
    };
}

impl_94!()