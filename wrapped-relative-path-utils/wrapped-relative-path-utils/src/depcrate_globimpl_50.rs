// Generated macro for impl_50 (impl)
macro_rules! Depcrate_globimpl_50 {
() => {
// Module: crate::glob
// Provides: {"impl_50"}
// Dependencies: {}
impl Iterator for Matcher < '_ > { type Item = Result < RelativePathBuf > ; fn next (& mut self) -> Option < Self :: Item > { 'outer : loop { let (mut path , mut components) = self . queue . pop_front () ? ; while let [first , rest @ ..] = components { match first { Component :: ParentDir => { path = path . join (relative_path :: Component :: ParentDir) ; } Component :: Normal (normal) => { path = path . join (normal) ; } Component :: Fragment (fragment) => { if let Err (e) = self . expand_filesystem (& path , rest , | name | fragment . is_match (name)) { return Some (Err (e)) ; } continue 'outer ; } Component :: StarStar => { if let Err (e) = self . walk (& path , rest) { return Some (Err (e)) ; } continue 'outer ; } } components = rest ; } return Some (Ok (path)) ; } } }
};
}
