// Generated macro for join_url (function)
macro_rules! Depcratejoin_url {
() => {
// Module: crate
// Provides: {"join_url"}
// Dependencies: {}
fn join_url < 'a > (base : Option < & Url > , dest : CowStr < 'a >) -> CowStr < 'a > { match base { Some (base_url) => { if dest . contains (':') || dest . starts_with ('#') { dest } else { let joined = base_url . join (& dest) . unwrap_or_else (| e | { panic ! ("failed to join URL `{}` to `{}`: {}" , dest , base_url , e) }) ; String :: from (joined) . into () } } None => dest , } }
};
}
