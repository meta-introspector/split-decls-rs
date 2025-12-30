// Generated macro for rustls_client_config_builder (struct)
macro_rules! Depcrate_clientrustls_client_config_builder {
() => {
// Module: crate::client
// Provides: {"rustls_client_config_builder"}
// Dependencies: {}
# [doc = " A client config being constructed."] # [doc = ""] # [doc = " A builder can be modified by, e.g. `rustls_client_config_builder_load_roots_from_file`."] # [doc = " Once you're done configuring settings, call `rustls_client_config_builder_build`"] # [doc = " to turn it into a *rustls_client_config."] # [doc = ""] # [doc = " Alternatively, if an error occurs or, you don't wish to build a config,"] # [doc = " call `rustls_client_config_builder_free` to free the builder directly."] # [doc = ""] # [doc = " This object is not safe for concurrent mutation. Under the hood,"] # [doc = " it corresponds to a `Box<ClientConfig>`."] # [doc = " <https://docs.rs/rustls/latest/rustls/struct.ConfigBuilder.html>"] pub struct rustls_client_config_builder { _private : [u8 ; 0] , }
};
}
