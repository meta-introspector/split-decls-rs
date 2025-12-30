// Generated macro for impl_265 (impl)
macro_rules! Depcrate_options_errorimpl_265 {
() => {
// Module: crate::options::error
// Provides: {"impl_265"}
// Dependencies: {}
impl OptionsError { # [doc = " Try to second-guess what the user was trying to do, depending on what"] # [doc = " went wrong."] pub fn suggestion (& self) -> Option < & 'static str > { match self { Self :: BadArgument (time , r) if * time == & flags :: TIME && r == "r" => { Some ("To sort oldest files last, try \"--sort oldest\", or just \"-sold\"") } Self :: Parse (ParseError :: NeedsValue { ref flag , .. }) if * flag == Flag :: Short (b't') => { Some ("To sort newest files last, try \"--sort newest\", or just \"-snew\"") } _ => None , } } }
};
}
