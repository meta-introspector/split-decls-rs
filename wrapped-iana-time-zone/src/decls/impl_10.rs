macro_rules! deps {
    () => {
        GetTimezoneError!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl std :: fmt :: Display for GetTimezoneError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { f . write_str (match self { GetTimezoneError :: FailedParsingString => "GetTimezoneError::FailedParsingString" , GetTimezoneError :: IoError (err) => return err . fmt (f) , GetTimezoneError :: OsError => "OsError" , }) } }
    };
}

impl_10!()