// Generated macro for ExtensionContext (struct)
macro_rules! Depcrate_extensionsExtensionContext {
() => {
// Module: crate::extensions
// Provides: {"ExtensionContext"}
// Dependencies: {}
# [doc = " Context for extension"] pub struct ExtensionContext < 'a > { # [doc = " Schema-scope context data, [`Registry`], and custom directives."] pub schema_env : & 'a SchemaEnv , # [doc = " Extension-scoped context data shared across all extensions."] # [doc = ""] # [doc = " Can be accessed only from hooks that implement the [`Extension`] trait."] # [doc = ""] # [doc = " It is created with each new [`Request`] and is empty by default."] # [doc = ""] # [doc = " For subscriptions, the session ends when the subscription is closed."] pub session_data : & 'a Data , # [doc = " Request-scoped context data shared across all resolvers."] # [doc = ""] # [doc = " This is a reference to [`Request::data`](Request) field."] # [doc = " If the request has not initialized yet, the value is seen as `None`"] # [doc = " inside the [`Extension::request`], [`Extension::subscribe`], and"] # [doc = " [`Extension::prepare_request`] hooks."] pub query_data : Option < & 'a Data > , }
};
}
