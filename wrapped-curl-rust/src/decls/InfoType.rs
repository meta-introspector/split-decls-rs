macro_rules! InfoType {
    () => {
        # [doc = " Possible data chunks that can be witnessed as part of the `debug_function`"] # [doc = " callback."] # [non_exhaustive] # [derive (Debug , Clone , Copy)] pub enum InfoType { # [doc = " The data is informational text."] Text , # [doc = " The data is header (or header-like) data received from the peer."] HeaderIn , # [doc = " The data is header (or header-like) data sent to the peer."] HeaderOut , # [doc = " The data is protocol data received from the peer."] DataIn , # [doc = " The data is protocol data sent to the peer."] DataOut , # [doc = " The data is SSL/TLS (binary) data received from the peer."] SslDataIn , # [doc = " The data is SSL/TLS (binary) data sent to the peer."] SslDataOut , }
    };
}

InfoType!();