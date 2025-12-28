macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! check_interpolation_result {
    () => {
        deps!();
        fn check_interpolation_result (disable : bool , res : Result < Cow < '_ , std :: path :: Path > , path :: interpolate :: Error > ,) -> Result < Option < Cow < '_ , std :: path :: Path > > , path :: interpolate :: Error > { if disable { return res . map (Some) ; } match res { Ok (good) => Ok (good . into ()) , Err (err) => match err { path :: interpolate :: Error :: Missing { .. } | path :: interpolate :: Error :: UserInterpolationUnsupported => { Ok (None) } path :: interpolate :: Error :: UsernameConversion (_) | path :: interpolate :: Error :: Utf8Conversion { .. } => { Err (err) } } , } }
    };
}

check_interpolation_result!()