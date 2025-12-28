macro_rules! deps {
    () => {
        BindIndex!();
        Statement!();
        Result!();
        Error!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl BindIndex for & '_ str { fn idx (& self , stmt : & Statement < '_ >) -> Result < usize > { match stmt . parameter_index (self) ? { Some (idx) => Ok (idx) , None => Err (Error :: InvalidParameterName (self . to_string ())) , } } }
    };
}

impl_38!();