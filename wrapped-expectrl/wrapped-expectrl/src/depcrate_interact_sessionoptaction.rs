// Generated macro for OptAction (type)
macro_rules! Depcrate_interact_sessionOptAction {
() => {
// Module: crate::interact::session
// Provides: {"OptAction"}
// Dependencies: {}
type OptAction < S , I , O , C > = Box < dyn FnMut (Context < '_ , S , I , O , C >) -> ExpectResult < bool > > ;
};
}
