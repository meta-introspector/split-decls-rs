// Generated macro for RouterConfig (struct)
macro_rules! Depcrate_configRouterConfig {
() => {
// Module: crate::config
// Provides: {"RouterConfig"}
// Dependencies: {}
# [doc = " Router configuration"] # [derive (Debug , Clone , Deserialize , Serialize)] pub struct RouterConfig { pub default : String , pub background : Option < String > , pub think : Option < String > , pub websearch : Option < String > , # [doc = " Regex pattern for auto-mapping models (e.g., \"^claude-\")."] # [doc = " If empty/null, defaults to Claude models only."] pub auto_map_regex : Option < String > , # [doc = " Regex pattern for detecting background tasks (e.g., \"(?i)claude.*haiku\")."] # [doc = " If empty/null, defaults to claude-haiku pattern."] pub background_regex : Option < String > , }
};
}
