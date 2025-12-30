// Generated macro for entries (function)
macro_rules! Depcrate_repository_odbentries {
() => {
// Module: crate::repository::odb
// Provides: {"entries"}
// Dependencies: {}
pub fn entries (repo : gix :: Repository , format : OutputFormat , mut out : impl io :: Write) -> anyhow :: Result < () > { if format != OutputFormat :: Human { bail ! ("Only human output format is supported at the moment") ; } for object in repo . objects . iter () ? { let object = object ? ; writeln ! (out , "{object}") ? ; } Ok (()) }
};
}
