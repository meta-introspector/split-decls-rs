// Generated macro for project (function)
macro_rules! Depcrateproject {
() => {
// Module: crate
// Provides: {"project"}
// Dependencies: {}
# [doc = " Generates a project layout, see [`ProjectBuilder`]"] pub fn project () -> ProjectBuilder { ProjectBuilder :: new (paths :: root () . join ("foo")) }
};
}
