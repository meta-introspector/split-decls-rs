macro_rules! deps {
    () => {
        GetTimezoneError!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl std :: error :: Error for GetTimezoneError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { GetTimezoneError :: FailedParsingString => None , GetTimezoneError :: IoError (err) => Some (err) , GetTimezoneError :: OsError => None , } } }
    };
}

impl_3!()