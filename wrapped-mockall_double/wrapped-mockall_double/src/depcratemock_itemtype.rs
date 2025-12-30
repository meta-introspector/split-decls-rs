// Generated macro for mock_itemtype (function)
macro_rules! Depcratemock_itemtype {
() => {
// Module: crate
// Provides: {"mock_itemtype"}
// Dependencies: {}
fn mock_itemtype (orig : & mut ItemType) { match & mut * orig . ty { Type :: Path (tp) => { let ident = & tp . path . segments . last_mut () . unwrap () . ident ; tp . path . segments . last_mut () . unwrap () . ident = mock_ident (ident) ; } x => compile_error (x . span () , "Only path types may be doubled") } }
};
}
