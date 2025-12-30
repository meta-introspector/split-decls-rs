// Generated macro for Error (enum)
macro_rules! Depcrate_errorsError {
() => {
// Module: crate::errors
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Password hashing errors."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub enum Error { # [doc = " Unsupported algorithm."] Algorithm , # [doc = " \"B64\" encoding error."] B64Encoding (B64Error) , # [doc = " Cryptographic error."] Crypto , # [doc = " Output size unexpected."] OutputSize { # [doc = " Indicates why the output size is unexpected."] # [doc = ""] # [doc = " - [`Ordering::Less`]: Size is too small."] # [doc = " - [`Ordering::Equal`]: Size is not exactly as `expected`."] # [doc = " - [`Ordering::Greater`]: Size is too long."] provided : Ordering , # [doc = " Expected output size in relation to `provided`."] # [doc = ""] # [doc = " - [`Ordering::Less`]: Minimum size."] # [doc = " - [`Ordering::Equal`]: Expected size."] # [doc = " - [`Ordering::Greater`]: Maximum size."] expected : usize , } , # [doc = " Duplicate parameter name encountered."] ParamNameDuplicated , # [doc = " Invalid parameter name."] ParamNameInvalid , # [doc = " Invalid parameter value."] ParamValueInvalid (InvalidValue) , # [doc = " Maximum number of parameters exceeded."] ParamsMaxExceeded , # [doc = " Invalid password."] Password , # [doc = " Password hash string invalid."] PhcStringField , # [doc = " Password hash string contains trailing data."] PhcStringTrailingData , # [doc = " Salt invalid."] SaltInvalid (InvalidValue) , # [doc = " Invalid algorithm version."] Version , # [doc = " Out of memory (heap allocation failure)."] OutOfMemory , }
};
}
