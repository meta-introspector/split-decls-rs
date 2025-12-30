// Generated macro for project_in (function)
macro_rules! Depcrateproject_in {
() => {
// Module: crate
// Provides: {"project_in"}
// Dependencies: {}
# [doc = " Generates a project layout in given directory, see [`ProjectBuilder`]"] pub fn project_in (dir : impl AsRef < Path >) -> ProjectBuilder { ProjectBuilder :: new (paths :: root () . join (dir) . join ("foo")) }
};
}
