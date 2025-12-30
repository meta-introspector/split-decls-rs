// Generated macro for FromEnv (struct)
macro_rules! DepcrateFromEnv {
() => {
// Module: crate
// Provides: {"FromEnv"}
// Dependencies: {}
# [doc = " Return type for [`Client::from_env_ext`] function."] # [derive (Debug)] pub struct FromEnv { # [doc = " Result of trying to get jobserver client from env."] pub client : Result < Client , FromEnvError > , # [doc = " Name and value of the environment variable."] # [doc = " `None` if no relevant environment variable is found."] pub var : Option < (& 'static str , OsString) > , }
};
}
