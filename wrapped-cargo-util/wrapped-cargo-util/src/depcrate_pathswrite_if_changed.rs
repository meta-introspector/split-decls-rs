// Generated macro for write_if_changed (function)
macro_rules! Depcrate_pathswrite_if_changed {
() => {
// Module: crate::paths
// Provides: {"write_if_changed"}
// Dependencies: {}
# [doc = " Equivalent to [`write()`], but does not write anything if the file contents"] # [doc = " are identical to the given contents."] pub fn write_if_changed < P : AsRef < Path > , C : AsRef < [u8] > > (path : P , contents : C) -> Result < () > { (| | -> Result < () > { let contents = contents . as_ref () ; let mut f = OpenOptions :: new () . read (true) . write (true) . create (true) . open (& path) ? ; let mut orig = Vec :: new () ; f . read_to_end (& mut orig) ? ; if orig != contents { f . set_len (0) ? ; f . seek (io :: SeekFrom :: Start (0)) ? ; f . write_all (contents) ? ; } Ok (()) }) () . with_context (| | format ! ("failed to write `{}`" , path . as_ref () . display ())) ? ; Ok (()) }
};
}
