// Generated macro for impl_59 (impl)
macro_rules! Depcrate_configimpl_59 {
() => {
// Module: crate::config
// Provides: {"impl_59"}
// Dependencies: {}
impl AppConfig { # [doc = " Get default config file path"] # [doc = " Returns ~/.claude-code-mux/config.toml (cross-platform)"] pub fn default_path () -> Result < PathBuf > { let home = dirs :: home_dir () . context ("Failed to get home directory") ? ; let config_dir = home . join (".claude-code-mux") ; std :: fs :: create_dir_all (& config_dir) . with_context (| | format ! ("Failed to create config directory: {}" , config_dir . display ())) ? ; Ok (config_dir . join ("config.toml")) } # [doc = " Load configuration from a TOML file"] pub fn from_file (path : & PathBuf) -> Result < Self > { if ! path . exists () { Self :: create_default_config (path) ? ; } let content = std :: fs :: read_to_string (path) . with_context (| | format ! ("Failed to read config file: {}" , path . display ())) ? ; let mut config : AppConfig = toml :: from_str (& content) . with_context (| | format ! ("Failed to parse config file: {}" , path . display ())) ? ; config . resolve_env_vars () ? ; Ok (config) } # [doc = " Create a default configuration file or migrate existing one"] fn create_default_config (path : & PathBuf) -> Result < () > { if let Some (parent) = path . parent () { std :: fs :: create_dir_all (parent) . with_context (| | format ! ("Failed to create config directory: {}" , parent . display ())) ? ; } let old_config_path = PathBuf :: from ("config/default.toml") ; if old_config_path . exists () { eprintln ! ("📦 Migrating existing config from {} to {}" , old_config_path . display () , path . display ()) ; std :: fs :: copy (& old_config_path , path) . with_context (| | format ! ("Failed to migrate config from {} to {}" , old_config_path . display () , path . display ())) ? ; eprintln ! ("✅ Migration complete! Your existing configuration has been preserved.") ; eprintln ! ("   Old location: {}" , old_config_path . display ()) ; eprintln ! ("   New location: {}" , path . display ()) ; eprintln ! () ; eprintln ! ("💡 You can safely delete the old config file if you want:") ; eprintln ! ("   rm {}" , old_config_path . display ()) ; } else { let default_config = Self :: default_config_content () ; std :: fs :: write (path , default_config) . with_context (| | format ! ("Failed to write default config file: {}" , path . display ())) ? ; eprintln ! ("Created default config file at: {}" , path . display ()) ; eprintln ! ("Please edit the config file to add your providers and models.") ; eprintln ! ("You can also configure via the web UI at http://127.0.0.1:13456") ; } Ok (()) } # [doc = " Generate default configuration content as TOML string"] fn default_config_content () -> String { r#"# Claude Code Mux Configuration
#
# This is a minimal default configuration.
# Configure your providers and models via the web UI at http://127.0.0.1:13456
# or edit this file directly.

[server]
host = "127.0.0.1"
port = 13456
log_level = "info"

[server.timeouts]
api_timeout_ms = 600000      # 10 minutes
connect_timeout_ms = 10000   # 10 seconds

[router]
# Default model to use when no routing conditions are met
# You MUST configure at least one provider and model before using CCM
default = "placeholder-model"

# Optional: Model for background tasks (e.g., "glm-4.5-air")
# background = ""

# Optional: Model for thinking/reasoning tasks (e.g., "claude-opus-4-1")
# think = ""

# Optional: Model for web search tasks (e.g., "glm-4.6")
# websearch = ""

# Optional: Regex pattern for auto-mapping models (e.g., "^claude-")
# auto_map_regex = ""

# Optional: Regex pattern for detecting background tasks (e.g., "(?i)claude.*haiku")
# background_regex = ""

# Providers configuration
# Add providers via the web UI or edit this section
# Example:
# [[providers]]
# name = "my-provider"
# provider_type = "anthropic"  # or "openai", "openrouter", etc.
# auth_type = "api_key"        # or "oauth"
# api_key = "your-api-key-here"
# enabled = true
# models = []

# Models configuration
# Add models via the web UI or edit this section
# Example:
# [[models]]
# name = "my-model"
#
# [[models.mappings]]
# provider = "my-provider"
# actual_model = "claude-sonnet-4-5"
# priority = 1
"# . to_string () } # [doc = " Resolve environment variables in configuration"] fn resolve_env_vars (& mut self) -> Result < () > { if let Some (ref key) = self . server . api_key { if key . starts_with ('$') { let env_var = & key [1 ..] ; self . server . api_key = std :: env :: var (env_var) . ok () ; } } for provider in & mut self . providers { if ! provider . is_enabled () { continue ; } if let Some (ref api_key) = provider . api_key { if api_key . starts_with ('$') { let env_var = & api_key [1 ..] ; if let Ok (value) = std :: env :: var (env_var) { provider . api_key = Some (value) ; } else { anyhow :: bail ! ("Environment variable {} not found for provider {}" , env_var , provider . name) ; } } } } Ok (()) } }
};
}
