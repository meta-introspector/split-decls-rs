macro_rules! deps {
    () => {
        Error!();
        ErrorKind!();
        Result!();
    };
}

macro_rules! specialize_err {
    () => {
        deps!();
        # [doc = " When the result is an error, transforms the ast::ErrorKind from the source"] # [doc = " Result into another one. This function is used to return clearer error"] # [doc = " messages when possible."] fn specialize_err < T > (result : Result < T > , from : ast :: ErrorKind , to : ast :: ErrorKind ,) -> Result < T > { if let Err (e) = result { if e . kind == from { Err (ast :: Error { kind : to , pattern : e . pattern , span : e . span }) } else { Err (e) } } else { result } }
    };
}

specialize_err!()