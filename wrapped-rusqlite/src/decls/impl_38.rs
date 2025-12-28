macro_rules! deps {
    () => {
        OptionalExtension!();
        Result!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < T > OptionalExtension < T > for Result < T > { fn optional (self) -> Result < Option < T > > { match self { Ok (value) => Ok (Some (value)) , Err (Error :: QueryReturnedNoRows) => Ok (None) , Err (e) => Err (e) , } } }
    };
}

impl_38!()