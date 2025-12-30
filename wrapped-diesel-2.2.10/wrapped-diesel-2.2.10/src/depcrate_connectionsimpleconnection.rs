// Generated macro for SimpleConnection (trait)
macro_rules! Depcrate_connectionSimpleConnection {
() => {
// Module: crate::connection
// Provides: {"SimpleConnection"}
// Dependencies: {}
# [doc = " Perform simple operations on a backend."] # [doc = ""] # [doc = " You should likely use [`Connection`] instead."] pub trait SimpleConnection { # [doc = " Execute multiple SQL statements within the same string."] # [doc = ""] # [doc = " This function is used to execute migrations,"] # [doc = " which may contain more than one SQL statement."] fn batch_execute (& mut self , query : & str) -> QueryResult < () > ; }
};
}
