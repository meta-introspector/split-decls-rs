// Generated macro for Protocol (enum)
macro_rules! DepcrateProtocol {
() => {
// Module: crate
// Provides: {"Protocol"}
// Dependencies: {}
# [doc = " SSL/TLS protocol versions."] # [derive (Debug , Copy , Clone)] pub enum Protocol { # [doc = " The SSL 3.0 protocol."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " SSL 3.0 has severe security flaws, and should not be used unless absolutely necessary. If"] # [doc = " you are not sure if you need to enable this protocol, you should not."] Sslv3 , # [doc = " The TLS 1.0 protocol."] Tlsv10 , # [doc = " The TLS 1.1 protocol."] Tlsv11 , # [doc = " The TLS 1.2 protocol."] Tlsv12 , # [doc (hidden)] __NonExhaustive , }
};
}
