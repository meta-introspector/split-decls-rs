// Generated macro for Error (enum)
macro_rules! Depcrate_file_init_from_envError {
() => {
// Module: crate::file::init::from_env
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Represents the errors that may occur when calling [`File::from_env()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Configuration {kind} at index {index} contained illformed UTF-8")] IllformedUtf8 { index : usize , kind : & 'static str } , # [error ("GIT_CONFIG_COUNT was not a positive integer: {}" , . input)] InvalidConfigCount { input : String } , # [error ("GIT_CONFIG_KEY_{} was not set" , . key_id)] InvalidKeyId { key_id : usize } , # [error ("GIT_CONFIG_KEY_{} was set to an invalid value: {}" , . key_id , . key_val)] InvalidKeyValue { key_id : usize , key_val : String } , # [error ("GIT_CONFIG_VALUE_{} was not set" , . value_id)] InvalidValueId { value_id : usize } , # [error (transparent)] PathInterpolationError (# [from] interpolate :: Error) , # [error (transparent)] Includes (# [from] init :: includes :: Error) , # [error (transparent)] Section (# [from] section :: header :: Error) , # [error (transparent)] ValueName (# [from] section :: value_name :: Error) , }
};
}
