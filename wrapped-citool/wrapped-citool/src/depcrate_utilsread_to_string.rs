// Generated macro for read_to_string (function)
macro_rules! Depcrate_utilsread_to_string {
() => {
// Module: crate::utils
// Provides: {"read_to_string"}
// Dependencies: {}
pub fn read_to_string < P : AsRef < Path > > (path : P) -> anyhow :: Result < String > { std :: fs :: read_to_string (& path) . with_context (| | format ! ("Cannot read file {:?}" , path . as_ref ())) }
};
}
