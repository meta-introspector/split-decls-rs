// Generated macro for BridgeConfig (struct)
macro_rules! Depcrate_bridgeBridgeConfig {
() => {
// Module: crate::bridge
// Provides: {"BridgeConfig"}
// Dependencies: {}
# [doc = " Configuration for establishing an active connection between a server and a"] # [doc = " client.  The server creates the bridge config (`run_server` in `server.rs`),"] # [doc = " then passes it to the client through the function pointer in the `run` field"] # [doc = " of `client::Client`. The client constructs a local `Bridge` from the config"] # [doc = " in TLS during its execution (`Bridge::{enter, with}` in `client.rs`)."] # [repr (C)] pub struct BridgeConfig < 'a > { # [doc = " Buffer used to pass initial input to the client."] input : Buffer , # [doc = " Server-side function that the client uses to make requests."] dispatch : closure :: Closure < 'a , Buffer , Buffer > , # [doc = " If 'true', always invoke the default panic hook"] force_show_panics : bool , _marker : marker :: PhantomData < * mut () > , }
};
}
