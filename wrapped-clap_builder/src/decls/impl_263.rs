macro_rules! deps {
    () => {
        ValueParser!();
        ValueParserInner!();
        Error!();
        Result!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl std :: fmt :: Debug for ValueParser { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { match & self . 0 { ValueParserInner :: Bool => f . debug_struct ("ValueParser::bool") . finish () , ValueParserInner :: String => f . debug_struct ("ValueParser::string") . finish () , ValueParserInner :: OsString => f . debug_struct ("ValueParser::os_string") . finish () , ValueParserInner :: PathBuf => f . debug_struct ("ValueParser::path_buf") . finish () , ValueParserInner :: Other (o) => write ! (f , "ValueParser::other({:?})" , o . type_id ()) , } } }
    };
}

impl_263!();