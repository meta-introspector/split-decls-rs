// Generated macro for get_status (function)
macro_rules! Depcrate_interact_sessionget_status {
() => {
// Module: crate::interact::session
// Provides: {"get_status"}
// Dependencies: {}
# [cfg (unix)] fn get_status < S > (session : & S) -> Result < Option < S :: Status > , Error > where S : Healthcheck , { match session . get_status () { Ok (status) => Ok (Some (status)) , Err (err) if err . kind () == ErrorKind :: WouldBlock => Ok (None) , Err (err) => Err (Error :: IO (err)) , } }
};
}
