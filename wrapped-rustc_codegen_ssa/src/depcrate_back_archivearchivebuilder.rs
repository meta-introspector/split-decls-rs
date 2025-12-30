// Generated macro for ArchiveBuilder (trait)
macro_rules! Depcrate_back_archiveArchiveBuilder {
() => {
// Module: crate::back::archive
// Provides: {"ArchiveBuilder"}
// Dependencies: {}
pub trait ArchiveBuilder { fn add_file (& mut self , path : & Path) ; fn add_archive (& mut self , archive : & Path , skip : Box < dyn FnMut (& str) -> bool + 'static > ,) -> io :: Result < () > ; fn build (self : Box < Self > , output : & Path) -> bool ; }
};
}
