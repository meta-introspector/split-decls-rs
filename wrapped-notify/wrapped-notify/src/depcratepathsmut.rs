// Generated macro for PathsMut (trait)
macro_rules! DepcratePathsMut {
() => {
// Module: crate
// Provides: {"PathsMut"}
// Dependencies: {}
# [doc = " Providing methods for adding and removing paths to watch."] # [doc = ""] # [doc = " `Box<dyn PathsMut>` is created by [`Watcher::paths_mut`]. See its documentation for more."] pub trait PathsMut { # [doc = " Add a new path to watch. See [`Watcher::watch`] for more."] fn add (& mut self , path : & Path , recursive_mode : RecursiveMode) -> Result < () > ; # [doc = " Remove a path from watching. See [`Watcher::unwatch`] for more."] fn remove (& mut self , path : & Path) -> Result < () > ; # [doc = " Ensure added/removed paths are applied."] # [doc = ""] # [doc = " The behaviour of dropping a [`PathsMut`] without calling [`commit`] is unspecified."] # [doc = " The implementation is free to ignore the changes or not, and may leave the watcher in a started or stopped state."] fn commit (self : Box < Self >) -> Result < () > ; }
};
}
