// Generated macro for Connection (struct)
macro_rules! Depcrate_connectionConnection {
() => {
// Module: crate::connection
// Provides: {"Connection"}
// Dependencies: {}
# [doc = " This is really just a holder to allow us to send messages through a shared reference to the"] # [doc = " connection."] # [derive (Debug)] pub struct Connection { inner : RefCell < InnerConnection > , }
};
}
