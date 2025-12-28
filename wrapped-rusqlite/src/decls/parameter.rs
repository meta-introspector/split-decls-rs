macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! parameter {
    () => {
        deps!();
        # [doc = " `<param_name>=['\"]?<param_value>['\"]?` => `(<param_name>, <param_value>)`"] pub fn parameter (c_slice : & [u8]) -> Result < (& str , & str) > { let arg = std :: str :: from_utf8 (c_slice) ? . trim () ; match arg . split_once ('=') { Some ((key , value)) => { let param = key . trim () ; let value = dequote (value . trim ()) ; Ok ((param , value)) } _ => Err (Error :: ModuleError (format ! ("illegal argument: '{arg}'"))) , } }
    };
}

parameter!()