// Generated macro for ProviderRegistry (struct)
macro_rules! Depcrate_providers_registryProviderRegistry {
() => {
// Module: crate::providers::registry
// Provides: {"ProviderRegistry"}
// Dependencies: {}
# [doc = " Provider registry that manages all configured providers"] pub struct ProviderRegistry { # [doc = " Map of provider name -> provider instance"] providers : HashMap < String , Arc < Box < dyn AnthropicProvider > > > , # [doc = " Map of model name -> provider name for fast lookup"] model_to_provider : HashMap < String , String > , }
};
}
