// Generated macro for Connection (struct)
macro_rules! Depcrate_common_connectionConnection {
() => {
// Module: crate::common::connection
// Provides: {"Connection"}
// Dependencies: {}
# [doc = " `Connection` header, defined in"] # [doc = " [RFC7230](https://datatracker.ietf.org/doc/html/rfc7230#section-6.1)"] # [doc = ""] # [doc = " The `Connection` header field allows the sender to indicate desired"] # [doc = " control options for the current connection.  In order to avoid"] # [doc = " confusing downstream recipients, a proxy or gateway MUST remove or"] # [doc = " replace any received connection options before forwarding the"] # [doc = " message."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Connection        = 1#connection-option"] # [doc = " connection-option = token"] # [doc = ""] # [doc = " # Example values"] # [doc = " * `close`"] # [doc = " * `keep-alive`"] # [doc = " * `upgrade`"] # [doc = " ```"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::Connection;"] # [doc = ""] # [doc = " let keep_alive = Connection::keep_alive();"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct Connection (FlatCsv) ;
};
}
