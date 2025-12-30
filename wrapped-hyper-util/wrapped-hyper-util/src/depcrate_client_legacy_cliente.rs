// Generated macro for e (macro)
macro_rules! Depcrate_client_legacy_cliente {
() => {
// Module: crate::client::legacy::client
// Provides: {"e"}
// Dependencies: {}
macro_rules ! e { ($ kind : ident) => { Error { kind : ErrorKind ::$ kind , source : None , connect_info : None , } } ; ($ kind : ident , $ src : expr) => { Error { kind : ErrorKind ::$ kind , source : Some ($ src . into ()) , connect_info : None , } } ; }
};
}
