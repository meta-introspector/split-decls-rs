// Generated macro for R2D2Connection (trait)
macro_rules! Depcrate_r2d2R2D2Connection {
() => {
// Module: crate::r2d2
// Provides: {"R2D2Connection"}
// Dependencies: {}
# [doc = " A trait indicating a connection could be used inside a r2d2 pool"] pub trait R2D2Connection : Connection { # [doc = " Check if a connection is still valid"] fn ping (& mut self) -> QueryResult < () > ; # [doc = " Checks if the connection is broken and should not be reused"] # [doc = ""] # [doc = " This method should return only contain a fast non-blocking check"] # [doc = " if the connection is considered to be broken or not. See"] # [doc = " [ManageConnection::has_broken] for details."] # [doc = ""] # [doc = " The default implementation does not consider any connection as broken"] fn is_broken (& mut self) -> bool { false } }
};
}
