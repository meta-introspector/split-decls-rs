// Generated macro for write_file (function)
macro_rules! Depcrate_new_lintwrite_file {
() => {
// Module: crate::new_lint
// Provides: {"write_file"}
// Dependencies: {}
fn write_file < P : AsRef < Path > , C : AsRef < [u8] > > (path : P , contents : C) -> io :: Result < () > { fn inner (path : & Path , contents : & [u8]) -> io :: Result < () > { OpenOptions :: new () . write (true) . create_new (true) . open (path) ? . write_all (contents) } inner (path . as_ref () , contents . as_ref ()) . context (format ! ("writing to file: {}" , path . as_ref () . display ())) }
};
}
