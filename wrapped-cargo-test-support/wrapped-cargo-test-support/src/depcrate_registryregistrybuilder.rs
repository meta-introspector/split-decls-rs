// Generated macro for RegistryBuilder (struct)
macro_rules! Depcrate_registryRegistryBuilder {
() => {
// Module: crate::registry
// Provides: {"RegistryBuilder"}
// Dependencies: {}
# [doc = " Prepare a local [`TestRegistry`] fixture"] # [doc = ""] # [doc = " See also [`init`] and [`alt_init`]"] pub struct RegistryBuilder { # [doc = " If set, configures an alternate registry with the given name."] alternative : Option < String > , # [doc = " The authorization token for the registry."] token : Option < Token > , # [doc = " If set, the registry requires authorization for all operations."] auth_required : bool , # [doc = " If set, serves the index over http."] http_index : bool , # [doc = " If set, serves the API over http."] http_api : bool , # [doc = " If set, config.json includes 'api'"] api : bool , # [doc = " Write the token in the configuration."] configure_token : bool , # [doc = " Write the registry in configuration."] configure_registry : bool , # [doc = " API responders."] custom_responders : HashMap < String , RequestCallback > , # [doc = " Handler for 404 responses."] not_found_handler : RequestCallback , # [doc = " If nonzero, the git index update to be delayed by the given number of seconds."] delayed_index_update : usize , # [doc = " Credential provider in configuration"] credential_provider : Option < String > , }
};
}
