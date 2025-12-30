// Generated macro for server (function)
macro_rules! Depcrateserver {
() => {
// Module: crate
// Provides: {"server"}
// Dependencies: {}
fn server (conn : & mut Connection) -> & mut ServerConnection { match conn { Connection :: Server (s) => s , _ => panic ! ("Connection is not a ServerConnection") , } }
};
}
