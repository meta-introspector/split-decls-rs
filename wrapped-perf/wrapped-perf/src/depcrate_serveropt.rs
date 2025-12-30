// Generated macro for Opt (struct)
macro_rules! Depcrate_serverOpt {
() => {
// Module: crate::server
// Provides: {"Opt"}
// Dependencies: {}
# [derive (Parser)] # [clap (name = "server")] pub struct Opt { # [doc = " Address to listen on"] # [clap (long = "listen" , default_value = "[::]:4433")] listen : SocketAddr , # [doc = " TLS private key in DER format"] # [clap (short = 'k' , long = "key" , requires = "cert")] key : Option < PathBuf > , # [doc = " TLS certificate in PEM format"] # [clap (short = 'c' , long = "cert" , requires = "key")] cert : Option < PathBuf > , # [doc = " Common options"] # [command (flatten)] common : CommonOpt , }
};
}
