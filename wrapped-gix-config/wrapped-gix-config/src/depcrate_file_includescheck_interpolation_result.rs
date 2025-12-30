// Generated macro for check_interpolation_result (function)
macro_rules! Depcrate_file_includescheck_interpolation_result {
() => {
// Module: crate::file::includes
// Provides: {"check_interpolation_result"}
// Dependencies: {}
fn check_interpolation_result (disable : bool , res : Result < Cow < '_ , std :: path :: Path > , path :: interpolate :: Error > ,) -> Result < Option < Cow < '_ , std :: path :: Path > > , path :: interpolate :: Error > { if disable { return res . map (Some) ; } match res { Ok (good) => Ok (good . into ()) , Err (err) => match err { path :: interpolate :: Error :: Missing { .. } | path :: interpolate :: Error :: UserInterpolationUnsupported => { Ok (None) } path :: interpolate :: Error :: UsernameConversion (_) | path :: interpolate :: Error :: Utf8Conversion { .. } => { Err (err) } } , } }
};
}
