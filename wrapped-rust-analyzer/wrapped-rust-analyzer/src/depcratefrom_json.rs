// Generated macro for from_json (function)
macro_rules! Depcratefrom_json {
() => {
// Module: crate
// Provides: {"from_json"}
// Dependencies: {}
pub fn from_json < T : DeserializeOwned > (what : & 'static str , json : & serde_json :: Value ,) -> anyhow :: Result < T > { serde_json :: from_value (json . clone ()) . map_err (| e | anyhow :: format_err ! ("Failed to deserialize {what}: {e}; {json}")) }
};
}
