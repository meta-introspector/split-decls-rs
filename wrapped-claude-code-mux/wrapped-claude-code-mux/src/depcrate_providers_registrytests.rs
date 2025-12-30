// Generated macro for tests (module)
macro_rules! Depcrate_providers_registrytests {
() => {
// Module: crate::providers::registry
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_empty_registry () { let registry = ProviderRegistry :: new () ; assert ! (registry . list_models () . is_empty ()) ; assert ! (registry . list_providers () . is_empty ()) ; } # [test] fn test_get_provider_for_model_not_found () { let registry = ProviderRegistry :: new () ; let result = registry . get_provider_for_model ("gpt-4") ; assert ! (result . is_err ()) ; } }
};
}
