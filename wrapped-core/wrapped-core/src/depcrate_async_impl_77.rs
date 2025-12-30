// Generated macro for impl_77 (impl)
macro_rules! Depcrate_async_impl_77 {
() => {
// Module: crate::async_
// Provides: {"impl_77"}
// Dependencies: {}
impl Async { fn parse (s : & str) -> Async { let (s , enabled) = match s . strip_prefix ('-') { Some (s) => (s , false) , None => (s , true) , } ; let filter = match s { "all" => AsyncFilter :: All , other => match other . strip_prefix ("import:") { Some (s) => AsyncFilter :: Import (s . to_string ()) , None => match other . strip_prefix ("export:") { Some (s) => AsyncFilter :: Export (s . to_string ()) , None => AsyncFilter :: Function (s . to_string ()) , } , } , } ; Async { enabled , filter } } }
};
}
