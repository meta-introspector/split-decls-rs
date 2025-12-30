// Generated macro for generate_url (function)
macro_rules! Depcrate_registrygenerate_url {
() => {
// Module: crate::registry
// Provides: {"generate_url"}
// Dependencies: {}
fn generate_url (name : & str) -> Url { Url :: from_file_path (generate_path (name)) . ok () . unwrap () }
};
}
