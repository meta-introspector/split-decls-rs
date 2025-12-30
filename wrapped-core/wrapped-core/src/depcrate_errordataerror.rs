// Generated macro for DataError (struct)
macro_rules! Depcrate_errorDataError {
() => {
// Module: crate::error
// Provides: {"DataError"}
// Dependencies: {}
# [doc = " The error type for ICU4X data provider operations."] # [doc = ""] # [doc = " To create one of these, either start with a [`DataErrorKind`] or use [`DataError::custom()`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Create a IdentifierNotFound error and attach a data request for context:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use icu_provider::prelude::*;"] # [doc = " let marker: DataMarkerInfo = unimplemented!();"] # [doc = " let req: DataRequest = unimplemented!();"] # [doc = " DataErrorKind::IdentifierNotFound.with_req(marker, req);"] # [doc = " ```"] # [doc = ""] # [doc = " Create a named custom error:"] # [doc = ""] # [doc = " ```"] # [doc = " # use icu_provider::prelude::*;"] # [doc = " DataError::custom(\"This is an example error\");"] # [doc = " ```"] # [derive (Clone , Copy , Eq , PartialEq , Debug)] # [non_exhaustive] pub struct DataError { # [doc = " Broad category of the error."] pub kind : DataErrorKind , # [doc = " The data marker of the request, if available."] pub marker : Option < DataMarkerId > , # [doc = " Additional context, if available."] pub str_context : Option < & 'static str > , # [doc = " Whether this error was created in silent mode to not log."] pub silent : bool , }
};
}
