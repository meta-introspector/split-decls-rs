// Generated macro for world_name (function)
macro_rules! Depcrateworld_name {
() => {
// Module: crate
// Provides: {"world_name"}
// Dependencies: {}
fn world_name (resolve : & Resolve , world : WorldId) -> String { format ! ("world.{}" , resolve . worlds [world] . name . to_lower_camel_case ()) }
};
}
