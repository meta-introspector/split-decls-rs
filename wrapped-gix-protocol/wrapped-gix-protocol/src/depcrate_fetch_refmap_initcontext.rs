// Generated macro for Context (struct)
macro_rules! Depcrate_fetch_refmap_initContext {
() => {
// Module: crate::fetch::refmap::init
// Provides: {"Context"}
// Dependencies: {}
# [doc = " For use in [`RefMap::fetch()`]."] # [derive (Debug , Clone)] pub struct Context { # [doc = " All explicit refspecs to identify references on the remote that you are interested in."] # [doc = " Note that these are copied to [`RefMap::refspecs`] for convenience, as `RefMap::mappings` refer to them by index."] pub fetch_refspecs : Vec < gix_refspec :: RefSpec > , # [doc = " A list of refspecs to use as implicit refspecs which won't be saved or otherwise be part of the remote in question."] # [doc = ""] # [doc = " This is useful for handling `remote.<name>.tagOpt` for example."] pub extra_refspecs : Vec < gix_refspec :: RefSpec > , }
};
}
