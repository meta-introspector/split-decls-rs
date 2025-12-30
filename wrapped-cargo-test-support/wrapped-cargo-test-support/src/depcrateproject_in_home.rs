// Generated macro for project_in_home (function)
macro_rules! Depcrateproject_in_home {
() => {
// Module: crate
// Provides: {"project_in_home"}
// Dependencies: {}
# [doc = " Generates a project layout inside our fake home dir, see [`ProjectBuilder`]"] pub fn project_in_home (name : impl AsRef < Path >) -> ProjectBuilder { ProjectBuilder :: new (paths :: home () . join (name)) }
};
}
