// Generated macro for Vars (trait)
macro_rules! Depcrate_options_varsVars {
() => {
// Module: crate::options::vars
// Provides: {"Vars"}
// Dependencies: {}
# [doc = " Mockable wrapper for `std::env::var_os`."] pub trait Vars { fn get (& self , name : & 'static str) -> Option < OsString > ; # [doc = " Get the variable `name` and if not set get the variable `fallback`."] fn get_with_fallback (& self , name : & 'static str , fallback : & 'static str) -> Option < OsString > { self . get (name) . or_else (| | self . get (fallback)) } # [doc = " Get the source of the value.  If the variable `name` is set return"] # [doc = " `Some(name)` else if the variable `fallback` is set return"] # [doc = " `Some(fallback)` else `None`."] fn source (& self , name : & 'static str , fallback : & 'static str) -> Option < & 'static str > { match self . get (name) { Some (_) => Some (name) , None => self . get (fallback) . and (Some (fallback)) , } } }
};
}
