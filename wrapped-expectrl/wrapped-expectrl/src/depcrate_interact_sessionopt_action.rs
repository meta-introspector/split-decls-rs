// Generated macro for opt_action (function)
macro_rules! Depcrate_interact_sessionopt_action {
() => {
// Module: crate::interact::session
// Provides: {"opt_action"}
// Dependencies: {}
fn opt_action < S , I , O , C > (ctx : Context < '_ , S , I , O , C > , opt : & mut Option < OptAction < S , I , O , C > > ,) -> ExpectResult < bool > { match opt { Some (action) => (action) (ctx) , None => Ok (false) , } }
};
}
