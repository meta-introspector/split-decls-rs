// Generated macro for ConfigureRequestFn (type)
macro_rules! Depcrate_client_blocking_io_http_reqwestConfigureRequestFn {
() => {
// Module: crate::client::blocking_io::http::reqwest
// Provides: {"ConfigureRequestFn"}
// Dependencies: {}
# [doc = " A function to configure a single request prior to sending it, support most complex configuration beyond what's possible with"] # [doc = " basic `git` http configuration."] pub type ConfigureRequestFn = dyn FnMut (& mut reqwest :: blocking :: Request) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > + Send + Sync + 'static ;
};
}
