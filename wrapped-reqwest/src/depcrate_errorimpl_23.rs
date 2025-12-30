// Generated macro for impl_23 (impl)
macro_rules! Depcrate_errorimpl_23 {
() => {
// Module: crate::error
// Provides: {"impl_23"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . inner . kind { Kind :: Builder => f . write_str ("builder error") ? , Kind :: Request => f . write_str ("error sending request") ? , Kind :: Body => f . write_str ("request or response body error") ? , Kind :: Decode => f . write_str ("error decoding response body") ? , Kind :: Redirect => f . write_str ("error following redirect") ? , Kind :: Upgrade => f . write_str ("error upgrading connection") ? , # [cfg (target_arch = "wasm32")] Kind :: Status (ref code) => { let prefix = if code . is_client_error () { "HTTP status client error" } else { debug_assert ! (code . is_server_error ()) ; "HTTP status server error" } ; write ! (f , "{prefix} ({code})") ? ; } # [cfg (not (target_arch = "wasm32"))] Kind :: Status (ref code , ref reason) => { let prefix = if code . is_client_error () { "HTTP status client error" } else { debug_assert ! (code . is_server_error ()) ; "HTTP status server error" } ; if let Some (reason) = reason { write ! (f , "{prefix} ({} {})" , code . as_str () , Escape :: new (reason . as_bytes ())) ? ; } else { write ! (f , "{prefix} ({code})") ? ; } } } ; if let Some (url) = & self . inner . url { write ! (f , " for url ({url})") ? ; } Ok (()) } }
};
}
