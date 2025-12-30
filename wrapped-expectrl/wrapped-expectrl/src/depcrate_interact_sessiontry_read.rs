// Generated macro for try_read (function)
macro_rules! Depcrate_interact_sessiontry_read {
() => {
// Module: crate::interact::session
// Provides: {"try_read"}
// Dependencies: {}
# [cfg (all (not (feature = "async") , not (feature = "polling")))] fn try_read < S > (session : & mut S , buf : & mut [u8]) -> ExpectResult < Option < usize > > where S : NonBlocking + Read , { session . set_blocking (false) ? ; let result = session . read (buf) ; session . set_blocking (true) ? ; match result { Ok (n) => Ok (Some (n)) , Err (err) if err . kind () == ErrorKind :: WouldBlock => Ok (None) , Err (err) => Err (Error :: IO (err)) , } }
};
}
